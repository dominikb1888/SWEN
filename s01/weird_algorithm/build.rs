use std::env;
use std::fs;
use std::io::Cursor;
use std::path::Path;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use scraper::{Html, Selector};

fn main() {
    dotenvy::dotenv().ok();

    let test_dir = "tests/inout";
    
    // 1. Get Project/Binary Name dynamically
    let bin_name = env::var("CARGO_PKG_NAME").expect("Failed to get package name");
    
    let problem_id = env::var("CSES_PROBLEM_ID").expect("CSES_PROBLEM_ID not set");
    let username = env::var("CSES_USERNAME").expect("CSES_USER not set");
    let password = env::var("CSES_PASSWORD").expect("CSES_PASS not set");

    if !Path::new(test_dir).exists() || fs::read_dir(test_dir).map(|d| d.count()).unwrap_or(0) == 0 {
        fs::create_dir_all(test_dir).unwrap();
        download_tests(&problem_id, &username, &password, test_dir);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let generated_test_file = Path::new(&out_dir).join("generated_tests.rs");

    let entries = fs::read_dir(test_dir).expect("Failed to read test directory");
    let mut test_cases = vec![];

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("in") {
            let output_file = path.with_extension("out");
            if output_file.exists() {
                test_cases.push((path, output_file));
            }
        }
    }

    // 2. Load and Replace Placeholders
    let test_template = fs::read_to_string("test_template.txt")
        .expect("Failed to read test template file");

    let mut test_code = String::new();
    // We no longer need 'use super::*' because this will run in an integration test file
    test_code.push_str("use assert_cmd::Command;\n\n");

    for (input_file, output_file) in test_cases {
        let test_name = input_file
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace("-", "_");

        let input_content = fs::read_to_string(&input_file).expect("Failed to read input file");
        let expected_output = fs::read_to_string(&output_file).expect("Failed to read output file");

        // Format the template: replace {bin_name} with the actual pkg name
        let formatted_test = test_template
            .replace("{test_name}", &test_name)
            .replace("{bin_name}", &bin_name) 
            .replace("{input_content}", &escape_string(&input_content))
            .replace("{expected_output}", &escape_string(&expected_output))
            .replace("{input_file}", &input_file.display().to_string());
            
        test_code.push_str(&formatted_test);
    }

    fs::write(&generated_test_file, test_code).expect("Failed to write generated test file");
    
    // Tell cargo to rerun if tests or the template change
    println!("cargo:rerun-if-changed={}", test_dir);
    println!("cargo:rerun-if-changed=test_template.txt");
}

fn escape_string(s: &str) -> String {
    s.replace("\"", "\\\"")
}

fn download_tests(id: &str, user: &str, pass: &str, target: &str) {
    let client = Client::builder()
        .cookie_store(true)
        .user_agent("Mozilla/5.0 (Rust Build Script)")
        .redirect(Policy::limited(50))
        .build()
        .unwrap();

   
    // In your download_cses_tests function:

    // 1. LOGIN PHASE
    let login_url = "https://cses.fi/login";
    let login_page = client.get(login_url).send().unwrap().text().unwrap();
    let login_token = extract_token(&login_page); // Token #1

    let mut login_form = std::collections::HashMap::new();
    login_form.insert("csrf_token", login_token);
    login_form.insert("nick", user.to_string());
    login_form.insert("pass", pass.to_string());

    // Submit login - redirect policy handles the rest
    let _login_response = client.post(login_url)
        .form(&login_form)
        .send()
        .expect("Login request failed");

    // 2. DOWNLOAD PHASE (Requires a NEW token from the task page)
    let task_url = format!("https://cses.fi/problemset/tests/{}", id);
    let task_page = client.get(&task_url).send().unwrap().text().unwrap();

    // Critical: Extract the NEW token specifically for the download form
    let download_token = extract_token(&task_page); // Token #2

    let mut dl_form = std::collections::HashMap::new();
    dl_form.insert("csrf_token", download_token);
    dl_form.insert("download", "true".to_string());

    let mut response = client.post(&task_url)
        .form(&dl_form)
        .send()
        .expect("Download POST failed");

    // UNZIP
    let mut content = Vec::new();
    std::io::copy(&mut response, &mut content).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(content)).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = Path::new(target).join(file.name());
        let mut outfile = fs::File::create(&outpath).unwrap();
        std::io::copy(&mut file, &mut outfile).unwrap();
    }
}

fn extract_token(html: &str) -> String {
    let doc = Html::parse_document(html);
    // Use a more specific selector
    let sel = Selector::parse("input[name='csrf_token']").unwrap();
    
    let token = doc.select(&sel)
        .next()
        .map(|el| el.value().attr("value"))
        .flatten()
        .map(|s| s.to_string());

    match token {
        Some(t) => {
            println!("cargo:warning=Successfully extracted CSRF token: {}", t);
            t
        },
        None => {
            // Debug the raw HTML to stderr if extraction fails
            eprintln!("Failed HTML Content:\n{}", html);
            panic!("CSRF token not found - page structure might have changed or HTML is malformed");
        }
    }
}


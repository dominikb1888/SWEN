///
struct City {
    name: String,
    population: i64,
    country: String,
    monster_attack_risk: f64,
    flooding_risk: f64,
}
/// Given a list of cities and a test function,
/// return how many cities pass the test.
fn count_selected_cities(cities: &Vec<City>, test_fn: &dyn Fn(&City) -> bool) -> usize {
    // let mut count = 0;
    // for city in cities {
    //     if test_fn(city) {
    //        count += 1;
    //     }
    // }
    // count

    cities.iter().fold(0, |acc, city| if test_fn(city) { acc + 1 } else { acc })
}

/// An example of a test function. Note that the type of
/// this function is `fn(&City) -> bool`, the same as
// /// the `test_fn` argument to `count_selected_cities`.
// fn has_monster_attacks(city: &City) -> bool {
//     city.monster_attack_risk > 0.0
// }
//
// fn has_flooding(city: &City) -> bool {
//     city.flooding_risk > 0.0
// }

// How many cities are at risk for monster attack
fn main() -> () {
    let my_cities = vec!(
        City { name: "Pfarrkirchen".to_string(), population: 12000, country: "Germany".to_string(), monster_attack_risk: 0.1, flooding_risk: 0.0 },
        City { name: "Hamburg".to_string(), population: 1700000, country: "Germany".to_string(), monster_attack_risk: 0.0, flooding_risk: 0.1 }
    );

    let n = count_selected_cities(&my_cities, &|city: &City| city.flooding_risk > 0.0);
    println!(n)
}
// //// Alternativ code without passing a function around
// ///
// fn count_selected_cities(cities: &Vec<City>, test_fn: &str) -> usize {
//     let mut count = 0;
//     for city in cities {
//         if test_fn == "has_monster_attacks" {
//             if city.monster_attack_risk > 0.0 {
//                 count += 1;
//             }
//         }
//         if test_fn == "has_flooding" {
//             if city.flooding_risk > 0.0 {
//                 count += 1;
//             }
//         }
//         (...)
//     }
//     count
// }
//
//
//

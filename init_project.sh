#!/bin/bash

# Configuration file name
VALUES_FILE="my-values.toml"
PROBLEM_ID=$1

# 1. Check for required dependencies
echo "Checking dependencies..."
for cmd in cargo cses-cli cargo-generate; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "Error: $cmd is not installed."
        if [ "$cmd" == "cses-cli" ]; then
            echo "Install it using: cargo install cses-cli"
	elif [ "$cmd" == "cargo" ]; then
	    echo "Cargo not available, please make sure you setup a proper Rust environment"
        elif [ "$cmd" == "cargo-generate" ]; then
            echo "Install it using: cargo install cargo-generate"
        fi
        exit 1
    fi
done

if [ -z "$PROBLEM_ID" ]; then
    echo "Usage: $0 <problem_id>"
    exit 1
fi

# 1. Check if my-values.toml exists; if not, prompt for credentials
if [ ! -f "$VALUES_FILE" ]; then
    echo "Credentials file ($VALUES_FILE) not found."
    read -p "Enter your CSES username: " CSES_USERNAME
    read -s -p "Enter your CSES password: " CSES_PASSWORD
    echo "" # New line after secret input

    # Create the toml file
    cat <<EOF > "$VALUES_FILE"
[values]
cses_username = "$CSES_USERNAME"
cses_password = "$CSES_PASSWORD"
EOF
    echo "Created $VALUES_FILE. Add this file to your global .gitignore!"
fi

# 2. Fetch the problem name using cses-cli
echo "Fetching metadata for problem $PROBLEM_ID..."

# -F'|' sets the field separator to the pipe character
# We trim whitespace using xargs and then transform to lowercase/underscores
RAW_NAME=$(cses-cli list | grep " $PROBLEM_ID " | awk -F'|' '{print $2}' | xargs)

if [ -z "$RAW_NAME" ]; then
    echo "Warning: Could not find problem name for ID $PROBLEM_ID. Using ID as name."
    PROBLEM_NAME="problem_$PROBLEM_ID"
else
    # Transform: lowercase and replace spaces with underscores
    PROBLEM_NAME=$(echo "$RAW_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
fi

echo "Problem identified: $PROBLEM_NAME"

# 3. Run cargo generate
cargo generate --git https://github.com/dominikb1888/cses_template \
               --template-values-file "$VALUES_FILE" \
               --name "$PROBLEM_NAME" \
               -d problem_id="$PROBLEM_ID" \
               -d problem_name="$PROBLEM_NAME" \
               --force-git-init

# 4. Navigate into the project and run tests
# This triggers build.rs, which handles the CSRF token and test download
cd "$PROBLEM_NAME" || exit
echo "Building project and downloading tests..."
cargo test

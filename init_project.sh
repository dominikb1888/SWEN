#!/bin/bash

# Configuration file name
VALUES_FILE="${VALUES_FILE:-my-values.toml}"
# Encrypted with: echo "PAT" | openssl enc -aes-256-cbc -a -salt -pbkdf2 -pass pass:PASSWORD
ENCRYPTED_GITHUB_TOKEN="U2FsdGVkX1+Bsr5IkJPx2ZX6fM7zOclSIY5afoygqBwpLAol2n6E43Hk1b4kBZ15lNZiavwC0AYfu6K76I21qw=="

# 1. Check for required dependencies
echo "Checking dependencies..."
for cmd in cargo cses-cli cargo-generate openssl; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "Error: $cmd is not installed."
        # ... (rest of dependency checks)
        exit 1
    fi
done

# 1. Check if my-values.toml exists and contains the token; if not, prompt for credentials
if [ ! -f "$VALUES_FILE" ] || ! grep -q "github_token =" "$VALUES_FILE"; then
    echo "Required credentials or token missing in $VALUES_FILE."
    
    # Only prompt if values are not already in the file
    if [ -f "$VALUES_FILE" ]; then
        CSES_USERNAME=$(grep 'cses_username =' "$VALUES_FILE" | cut -d'"' -f2)
        CSES_PASSWORD=$(grep 'cses_password =' "$VALUES_FILE" | cut -d'"' -f2)
        GH_USERNAME=$(grep 'github_user =' "$VALUES_FILE" | cut -d'"' -f2)
    fi

    [ -z "$CSES_USERNAME" ] && read -p "Enter your CSES username: " CSES_USERNAME
    [ -z "$CSES_PASSWORD" ] && read -s -p "Enter your CSES password: " CSES_PASSWORD && echo
    
    if [ -z "$GH_USERNAME" ]; then
        read -p "Enter your GitHub username (optional, for personal access): " GH_USERNAME
    fi
    
    read -s -p "Enter the Classroom Access Token: " CLASSROOM_PASSWORD
    echo

    # Decrypt the GitHub Token
    GITHUB_TOKEN=$(echo "$ENCRYPTED_GITHUB_TOKEN" | openssl enc -aes-256-cbc -d -a -pbkdf2 -pass pass:"$CLASSROOM_PASSWORD" 2>/dev/null)

    if [ -z "$GITHUB_TOKEN" ]; then
        echo "Error: Invalid Classroom Access Token."
        exit 1
    fi

    # Create/Update the toml file
    cat <<EOF > "$VALUES_FILE"
[values]
github_org = "26S-SWEN"
github_user = "$GH_USERNAME"
cses_username = "$CSES_USERNAME"
cses_password = "$CSES_PASSWORD"
github_token = "$GITHUB_TOKEN"
EOF
    echo "Updated $VALUES_FILE with GitHub token."
fi

if [ -z "$1" ]; then
    echo "Usage: $0 <problem_id_or_name>"
    exit 1
fi

INPUT="$1"

# 2. Fetch the problem name and ID using cses-cli
echo "Fetching metadata for problem $INPUT..."

# Check if input is a numeric ID
if [[ "$INPUT" =~ ^[0-9]+$ ]]; then
    PROBLEM_ID="$INPUT"
    # -F'|' sets the field separator to the pipe character
    # We trim whitespace using xargs and then transform to lowercase/underscores
    RAW_NAME=$(cses-cli list | grep " $PROBLEM_ID " | awk -F'|' '{print $2}' | xargs)
else
    # Treat input as a name (might contain underscores or spaces)
    SEARCH_NAME=$(echo "$INPUT" | tr '_' ' ')
    MATCH_LINE=$(cses-cli list | grep -i "$SEARCH_NAME" | head -n 1)
    
    if [ -n "$MATCH_LINE" ]; then
        PROBLEM_ID=$(echo "$MATCH_LINE" | awk -F'|' '{print $1}' | xargs)
        RAW_NAME=$(echo "$MATCH_LINE" | awk -F'|' '{print $2}' | xargs)
    else
        echo "Error: Could not find problem matching '$INPUT'"
        exit 1
    fi
fi

if [ -z "$RAW_NAME" ]; then
    echo "Warning: Could not find problem name for ID $PROBLEM_ID. Using ID as name."
    PROBLEM_NAME="problem_$PROBLEM_ID"
else
    # Transform: lowercase and replace spaces with underscores
    PROBLEM_NAME=$(echo "$RAW_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
fi

echo "Problem identified: $PROBLEM_NAME (ID: $PROBLEM_ID)"

# 3. Run cargo generate
GIT_AUTHOR_NAME="swen-bot" GIT_AUTHOR_EMAIL="swen-bot@users.noreply.github.com" \
GIT_COMMITTER_NAME="swen-bot" GIT_COMMITTER_EMAIL="swen-bot@users.noreply.github.com" \
cargo generate --git git@github.com:dominikb1888/cses_template.git \
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

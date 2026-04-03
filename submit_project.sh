#!/bin/bash

# Configuration
VALUES_FILE="${VALUES_FILE:-my-values.toml}"

if [ -z "$1" ]; then
    echo "Usage: $0 <problem_id_or_directory> [file_path]"
    exit 1
fi

TARGET="$1"
FILE_ARG="$2"

# 1. Determine PROJECT_DIR and TASK_ID
if [ -d "$TARGET" ]; then
    PROJECT_DIR=$(readlink -f "$TARGET")
    if [ -f "$PROJECT_DIR/.env" ]; then
        TASK_ID=$(grep CSES_PROBLEM_ID "$PROJECT_DIR/.env" | cut -d'"' -f2)
    fi
elif [[ "$TARGET" =~ ^[0-9]+$ ]]; then
    TASK_ID="$TARGET"
    RAW_NAME=$(cses-cli list | grep " $TASK_ID " | awk -F'|' '{print $2}' | xargs)
    if [ -n "$RAW_NAME" ]; then
        PROBLEM_NAME=$(echo "$RAW_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
        if [ -d "$PROBLEM_NAME" ]; then
            PROJECT_DIR=$(readlink -f "$PROBLEM_NAME")
        fi
    fi
else
    # Treat TARGET as problem name
    SEARCH_NAME=$(echo "$TARGET" | tr '_' ' ')
    MATCH_LINE=$(cses-cli list | grep -i "$SEARCH_NAME" | head -n 1)
    if [ -n "$MATCH_LINE" ]; then
        TASK_ID=$(echo "$MATCH_LINE" | awk -F'|' '{print $1}' | xargs)
        RAW_NAME=$(echo "$MATCH_LINE" | awk -F'|' '{print $2}' | xargs)
        PROBLEM_NAME=$(echo "$RAW_NAME" | tr '[:upper:]' '[:lower:]' | tr ' ' '_')
        if [ -d "$PROBLEM_NAME" ]; then
            PROJECT_DIR=$(readlink -f "$PROBLEM_NAME")
        fi
    fi
fi

if [ -z "$TASK_ID" ]; then
    echo "Error: Could not determine Task ID."
    exit 1
fi

if [ -z "$PROJECT_DIR" ]; then
    echo "Error: Could not find project directory for '$TARGET'."
    exit 1
fi

# 2. Determine FILE to submit
if [ -n "$FILE_ARG" ]; then
    FILE="$FILE_ARG"
else
    FILE="$PROJECT_DIR/src/main.rs"
fi

if [ ! -f "$FILE" ]; then
    echo "Error: File $FILE not found."
    exit 1
fi

# 3. Find the values file
if [ ! -f "$VALUES_FILE" ]; then
    if [ -f "../$VALUES_FILE" ]; then
        VALUES_FILE="../$VALUES_FILE"
    else
        echo "Error: $VALUES_FILE not found."
        exit 1
    fi
fi

# 2. Check for GitHub Org and CSES username
GH_ORG=$(grep 'github_org =' "$VALUES_FILE" | cut -d'"' -f2)
GH_USER=$(grep 'github_user =' "$VALUES_FILE" | cut -d'"' -f2)
CSES_USER=$(grep 'cses_username =' "$VALUES_FILE" | cut -d'"' -f2)
GH_TOKEN=$(grep 'github_token =' "$VALUES_FILE" | cut -d'"' -f2)

# Export token for gh CLI
if [ -n "$GH_TOKEN" ] && [ "$GH_TOKEN" != "YOUR_GITHUB_TOKEN_HERE" ]; then
    export GH_TOKEN
fi

# Export token for gh CLI early to ensure all gh commands use it
if [ -n "$GH_TOKEN" ] && [ "$GH_TOKEN" != "YOUR_GITHUB_TOKEN_HERE" ]; then
    export GH_TOKEN
fi

if [ -z "$GH_ORG" ]; then
    read -p "Enter your GitHub Organization: " GH_ORG
    if grep -q "\[values\]" "$VALUES_FILE"; then
         sed -i "/\[values\]/a github_org = \"$GH_ORG\"" "$VALUES_FILE"
    else
         echo "github_org = \"$GH_ORG\"" >> "$VALUES_FILE"
    fi
    echo "Saved github_org to $VALUES_FILE"
fi

if [ -z "$CSES_USER" ]; then
    read -p "Enter your CSES username: " CSES_USER
    if grep -q "\[values\]" "$VALUES_FILE"; then
         sed -i "/\[values\]/a cses_username = \"$CSES_USER\"" "$VALUES_FILE"
    else
         echo "cses_username = \"$CSES_USER\"" >> "$VALUES_FILE"
    fi
    echo "Saved cses_username to $VALUES_FILE"
fi

# 3. Check for gh CLI
if ! command -v gh &> /dev/null; then
    echo "Error: gh (GitHub CLI) is not installed."
    exit 1
fi

# 4. Optional: Personal Invitation
if [ -n "$GH_USER" ] && [ "$GH_USER" != "SKIP" ]; then
    echo "Attempting to ensure $GH_USER is a member of $GH_ORG..."
    gh api -X PUT "/orgs/$GH_ORG/memberships/$GH_USER" --silent || true
fi

# 5. Check gh auth status (Skip if GH_TOKEN is set)
if [ -z "$GH_TOKEN" ] || [ "$GH_TOKEN" == "YOUR_GITHUB_TOKEN_HERE" ]; then
    if ! gh auth status &> /dev/null; then
        echo "Error: Not authenticated with GitHub. Please run 'gh auth login' or provide a token."
        exit 1
    fi
fi

# 5. Check access to organization (Skip if using a token)
if [ -z "$GH_TOKEN" ] || [ "$GH_TOKEN" == "YOUR_GITHUB_TOKEN_HERE" ]; then
    echo "Checking access to GitHub organization: $GH_ORG..."
    USER_LOGIN=$(gh api user --jq '.login')
    if [ "$GH_ORG" != "$USER_LOGIN" ]; then
        if ! gh api user/orgs --jq '.[].login' | grep -iq "^$GH_ORG$"; then
            echo "Error: You do not have access to the GitHub organization/user '$GH_ORG'."
            exit 1
        fi
    fi
fi

# 6. Prepare the repository
PROJECT_NAME=$(basename "$PROJECT_DIR")

echo "Project name: $PROJECT_NAME"

# 7. Ensure it's a git repo
if [ ! -d "$PROJECT_DIR/.git" ]; then
    echo "Initializing flat git repository in $PROJECT_DIR..."
    git -C "$PROJECT_DIR" init
fi

# Ensure an initial commit exists
if ! git -C "$PROJECT_DIR" rev-parse --verify HEAD &> /dev/null; then
    echo "Staging files and creating initial commit..."
    git -C "$PROJECT_DIR" add .
    GIT_AUTHOR_NAME="swen-bot" GIT_AUTHOR_EMAIL="swen-bot@users.noreply.github.com" \
    GIT_COMMITTER_NAME="swen-bot" GIT_COMMITTER_EMAIL="swen-bot@users.noreply.github.com" \
    git -C "$PROJECT_DIR" commit -m "initial"
fi

REPO_FULL_NAME="$GH_ORG/$PROJECT_NAME-$CSES_USER"

# Build Expected URL. If token is present, use HTTPS with token for easy pushing.
if [ -n "$GH_TOKEN" ] && [ "$GH_TOKEN" != "YOUR_GITHUB_TOKEN_HERE" ]; then
    EXPECTED_URL="https://x-access-token:$GH_TOKEN@github.com/$REPO_FULL_NAME.git"
else
    EXPECTED_URL="git@github.com:$REPO_FULL_NAME.git"
fi

CURRENT_REMOTE=$(git -C "$PROJECT_DIR" remote get-url origin 2>/dev/null)

# Fallback: if remote origin does not match the required pattern
if [ -n "$CURRENT_REMOTE" ] && ! echo "$CURRENT_REMOTE" | grep -q "$REPO_FULL_NAME"; then
    echo "Remote origin does not match $REPO_FULL_NAME. Checking GitHub..."
    if ! gh repo view "$REPO_FULL_NAME" &> /dev/null; then
        echo "Creating repository $REPO_FULL_NAME on GitHub..."
        gh repo create "$REPO_FULL_NAME" --private
    fi
    echo "Updating remote origin to $EXPECTED_URL..."
    git -C "$PROJECT_DIR" remote set-url origin "$EXPECTED_URL"
    CURRENT_REMOTE="$EXPECTED_URL"
fi

# Check if repo exists on GitHub
if ! gh repo view "$REPO_FULL_NAME" &> /dev/null; then
    echo "Creating repository $REPO_FULL_NAME on GitHub..."
    gh repo create "$REPO_FULL_NAME" --private
fi

# Ensure the user has admin rights to the repo (optional)
if [ -n "$GH_USER" ] && [ "$GH_USER" != "SKIP" ]; then
    echo "Attempting to grant $GH_USER admin rights to $REPO_FULL_NAME..."
    gh api -X PUT "/repos/$REPO_FULL_NAME/collaborators/$GH_USER" -f permission=admin --silent || true
fi

# Ensure remote origin is set correctly and uses the token
if [ -z "$(git -C "$PROJECT_DIR" remote get-url origin 2>/dev/null)" ]; then
    git -C "$PROJECT_DIR" remote add origin "$EXPECTED_URL"
else
    git -C "$PROJECT_DIR" remote set-url origin "$EXPECTED_URL"
fi

echo "Pushing current state to GitHub..."
git -C "$PROJECT_DIR" push -u origin "$(git -C "$PROJECT_DIR" rev-parse --abbrev-ref HEAD)"

REPO_URL=$(gh repo view "$REPO_FULL_NAME" --json url -q .url)

# 8. Insert URL into the file
# We want to add it as a comment at the top.
# Check if a GitHub Repository comment already exists
if grep -q "GitHub Repository:" "$FILE"; then
    # Update it if it changed
    sed -i "s|// GitHub Repository:.*|// GitHub Repository: $REPO_URL|" "$FILE"
else
    # Insert at the top
    sed -i "1i // GitHub Repository: $REPO_URL" "$FILE"
fi

# Commit and push the updated file if there are changes
RELATIVE_FILE=$(realpath --relative-to="$PROJECT_DIR" "$FILE")
if git -C "$PROJECT_DIR" status --short | grep -q "$RELATIVE_FILE"; then
    echo "Committing URL update..."
    git -C "$PROJECT_DIR" add "$RELATIVE_FILE"
    GIT_AUTHOR_NAME="swen-bot" GIT_AUTHOR_EMAIL="swen-bot@users.noreply.github.com" \
    GIT_COMMITTER_NAME="swen-bot" GIT_COMMITTER_EMAIL="swen-bot@users.noreply.github.com" \
    git -C "$PROJECT_DIR" commit -m "Add GitHub repository URL to $RELATIVE_FILE"
    git -C "$PROJECT_DIR" push
fi

# 9. Perform submission
echo "Submitting to CSES..."
cses-cli submit -c ckvo8q5wh -t "$TASK_ID" "$FILE"

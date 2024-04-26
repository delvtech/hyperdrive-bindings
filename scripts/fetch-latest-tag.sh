#!/bin/bash

# GitHub organization
ORG=$1

# GitHub repository name
REPO=$2

# GitHub Token provided by GitHub Actions
TOKEN=${GITHUB_TOKEN}

# Fetch all tags from the repository
TAG=$(curl -sH "Authorization: token $TOKEN" "https://api.github.com/repos/$ORG/$REPO/releases/latest" | jq -r '.tag_name')

# Sanitize repository name to create a valid environment variable name
# Replace '/' with '_' and convert to lower case
ENV_REPO_NAME=$(echo "$REPO" | tr '/' '_' | tr '[:upper:]' '[:lower:]')

# Output the tag for subsequent steps
echo "latest_tag_${ENV_REPO_NAME}=$TAG" >> $GITHUB_ENV

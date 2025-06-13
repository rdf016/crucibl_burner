#!/bin/bash
# Usage: Edit <your-username> below, then run: bash push_to_github.sh

set -e

# Initialize git repo if not already
if [ ! -d .git ]; then
  git init
fi

git add .
git commit -m "Initial Crucibl burner contract"

git branch -M main

git remote add origin https://github.com/<rdf016>/crucibl_burner.git || true

git push -u origin main

echo "\n✅ Project pushed! Now go to your repo on GitHub and launch a Codespace." 
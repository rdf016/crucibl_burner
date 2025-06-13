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

git remote remove origin || true

git remote add origin https://github.com/rdf016/crucibl_burner.git

cd ~/crucibl_burner
git pull origin main --allow-unrelated-histories

git add .
git commit -m "Merge remote main with local"

git push -u origin main

echo "\n✅ Project pushed! Now go to your repo on GitHub and launch a Codespace." 
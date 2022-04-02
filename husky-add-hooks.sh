#!/bin/sh

npx husky add .husky/commit-msg ".git/hooks/commit-msg \$1"
npx husky add .husky/pre-push "RUSTFLAGS=-Awarnings cargo test -q"

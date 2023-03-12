#!/usr/bin/env bash

git fetch

# update backend
cd backend
cargo b --release

# update frontend
cd ..
cd frontend
npm ci
npm run build
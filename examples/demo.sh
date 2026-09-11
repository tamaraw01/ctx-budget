#!/bin/bash
# Demo: ctx-budget token analysis

echo "=== ctx-budget Demo ==="
echo ""
echo "Analyzing current directory..."
../target/release/ctx-budget . --model gpt-4o --limit 5

echo ""
echo "Analyzing with Claude context window..."
../target/release/ctx-budget . --model claude-3-5-sonnet-20240620 --limit 5

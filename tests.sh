#!/bin/bash

# Ensure ./tests directory exists
if [ ! -d "./tests" ]; then
    echo "Error: ./tests directory does not exist."
    exit 1
fi
echo "-----------------------------"
# Iterate over all files in ./tests
for testfile in ./tests/*; do
    if [ -f "$testfile" ]; then
        # Run the first command
        echo "cse 6991 ortalab "$testfile" --explain"
        
        # Run the second command
        echo "cse 6991 cargo run "$testfile" --explain"

        echo "-----------------------------"
    fi
done
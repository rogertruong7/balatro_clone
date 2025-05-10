#!/bin/bash

# Check if a search string is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <search_string>"
    exit 1
fi

SEARCH_STRING="$1"
SRC_DIR="./src"

# Ensure the directory exists
if [ ! -d "$SRC_DIR" ]; then
    echo "Error: Directory $SRC_DIR does not exist."
    exit 1
fi

# Search for the string in all files inside ./src
grep -rnw "$SRC_DIR" -e "$SEARCH_STRING"
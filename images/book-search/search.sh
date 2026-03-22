#!/bin/sh
query=$(jq -r '.query')

if [ -z "$query" ]; then
    echo "Please enter a search term"
    exit 0
fi

grep -i -n "$query" /book.txt

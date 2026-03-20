#!/bin/sh
a=$1
b=$2
op=$3

if [ "$op" = "add" ]; then
    result=$(echo "$a + $b" | bc -l)
elif [ "$op" = "multiply" ]; then
    result=$(echo "$a * $b" | bc -l)
else
    echo "{\"error\":\"unknown op: $op\"}"
    exit 1
fi

echo "{\"result\":$result}"

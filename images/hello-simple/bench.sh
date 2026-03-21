#!/bin/sh
read input
i=0
while [ $i -lt 50 ]; do
    echo "{\"event\":\"output\",\"data\":{\"word\":\"word$i\"}}"
    i=$((i + 1))
done
echo "{\"event\":\"output\",\"data\":{\"done\":true}}"

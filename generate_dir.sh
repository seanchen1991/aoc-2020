#!/bin/sh

echo "\033[0;32mGenerating directory for AoC day" $1

directory="day$1"

mkdir "$directory"

cp -r template/ "$directory"


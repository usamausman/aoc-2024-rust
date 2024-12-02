#!/usr/bin/env bash

set -euo pipefail

DAY=$1

echo "Getting input for day $DAY..."

mkdir -p input/day_"$DAY"
curl --silent --cookie "session=$(cat .token)" -o input/day_"$DAY"/in https://adventofcode.com/2024/day/"$DAY"/input

echo "Got input!"

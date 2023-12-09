#!/usr/bin/env bash

next_day=
for day in {1..25}; do
    day2d=$(printf "%02d" $day)
    if [ ! -d "$day2d" ]; then
        next_day="$day2d"
        break
    fi
done

if [ -z "$next_day" ]; then
    echo "No more days to create workspaces for."
    exit 1
fi

cargo new --name "day$next_day" "$next_day"
cd "$next_day" || exit 7
touch example_input.txt
touch input.txt

cat <<EOF > src/main.rs
use std::{env, fs};

fn main() {
    let input_file_path = env::args().nth(1).unwrap_or("$next_day/example_input.txt".into());
    let input = fs::read_to_string(&input_file_path).unwrap();
    dbg!(input);
}
EOF

#!/bin/sh

day_num="$1"
new_dir="src/day_$1"

cat >> Cargo.toml << EOF

[[bin]]
name = "day_${day_num}"
path = "${new_dir}/${day_num}.rs"
EOF

mkdir $new_dir && cd $new_dir
touch "$1.in" "$1.ex" "$1.rs"

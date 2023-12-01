#!/bin/sh

if [ -z "$1" ]
then
  cat << EOF
Usage: new_day.sh <day_number>

Example: new_day.sh 08
EOF
  exit 1
fi

day_number=$1

if [ -d "day$day_number" ]
then
  echo "Package day$day_number already exists"
  exit 1
fi

echo "> Creating package day$day_number"
cargo new "day$day_number"

echo "> Adding day$day_number to the workspace"
# Add day$day_number to the workspace members list
sed -i "s/\(members = \[.*\)/\1\n  \"day$day_number\",/" Cargo.toml

echo "> Copying template from day00"
cp -r day00/src "day$day_number"
# Replace day00 with day$day_number in the template main function
sed -i "s/day00/day$day_number/g" "day$day_number/src/main.rs"

echo "> Creating empty input files"
touch day$day_number/part{1,2}.{input,example}.txt

echo "Use \`cargo run -p day$day_number\` to run the code"

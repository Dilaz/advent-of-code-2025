#!/bin/bash
set -e

if [ -z "$1" ]; then
    echo "Usage: ./new-day.sh <day_number>"
    echo "Example: ./new-day.sh 1"
    exit 1
fi

DAY=$1

if ! [[ "$DAY" =~ ^[0-9]+$ ]] || [ "$DAY" -lt 1 ] || [ "$DAY" -gt 25 ]; then
    echo "Error: Day must be a number between 1 and 25"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SRC_DIR="$SCRIPT_DIR/src/bin/day$DAY"
INPUT_FILE="$SCRIPT_DIR/inputs/day$DAY.txt"

if [ -d "$SRC_DIR" ]; then
    echo "Error: day$DAY already exists at $SRC_DIR"
    exit 1
fi

# Copy template
cp -r "$SCRIPT_DIR/src/bin/template" "$SRC_DIR"
chmod -R u+w "$SRC_DIR"

# Replace DayX with Day{N} and dayX.txt with day{N}.txt
for file in "$SRC_DIR"/*.rs; do
    sed -i "s/DayX/Day$DAY/g" "$file"
    sed -i "s/dayX\.txt/day$DAY.txt/g" "$file"
done

# Add bin and bench entries to Cargo.toml if not exists
if ! grep -q "name = \"day$DAY\"" "$SCRIPT_DIR/Cargo.toml"; then
    cat >> "$SCRIPT_DIR/Cargo.toml" << EOF

[[bin]]
name = "day$DAY"
bench = false
test = true

[[bench]]
name = "day$DAY"
harness = false
path = "src/bin/day$DAY/bench.rs"
EOF
fi

# Create empty input file
mkdir -p "$SCRIPT_DIR/inputs"
touch "$INPUT_FILE"

echo "Created day$DAY successfully!"
echo ""
echo "Next steps:"
echo "  1. Add your puzzle input to: inputs/day$DAY.txt"
echo "  2. Edit solution: src/bin/day$DAY/solution.rs"
echo "  3. Run:   cargo run --bin day$DAY"
echo "  4. Test:  cargo test --bin day$DAY"
echo "  5. Bench: cargo bench --bench day$DAY"

#!/bin/bash
# Compare Slate Engine benchmarks with baseline
#
# Usage:
#   ./scripts/compare_benchmarks.sh <baseline-name>
#
# Example:
#   # Save current as baseline
#   cargo bench --workspace -- --save-baseline main
#   
#   # Make changes...
#   
#   # Compare with baseline
#   ./scripts/compare_benchmarks.sh main

set -e

BASELINE="${1:-main}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Benchmark Comparison                 ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "Comparing against baseline: ${GREEN}${BASELINE}${NC}"
echo ""

# Check if baseline exists
if [ ! -d "target/criterion" ]; then
    echo -e "${RED}✗ No benchmark data found${NC}"
    echo "  Run benchmarks first: cargo bench --workspace"
    exit 1
fi

# Run comparison
echo -e "${YELLOW}Running benchmarks with comparison...${NC}"
echo ""

cargo bench --workspace -- --baseline "$BASELINE"

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  Comparison Complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Detailed results: target/criterion/report/index.html"
echo ""

# Parse and display summary (if critcmp is installed)
if command -v critcmp &> /dev/null; then
    echo -e "${BLUE}Summary (via critcmp):${NC}"
    echo ""
    critcmp "$BASELINE" new
else
    echo -e "${YELLOW}Tip: Install critcmp for better comparison output${NC}"
    echo "  cargo install critcmp"
fi

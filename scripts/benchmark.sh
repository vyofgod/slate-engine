#!/bin/bash
# Slate Engine Benchmark Runner

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   Slate Engine Benchmarks              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

echo -e "${GREEN}→ Running slate-kernel benchmarks${NC}"
cargo bench -p slate-kernel
echo ""

echo -e "${GREEN}→ Running slate-benchmarks${NC}"
cargo bench -p slate-benchmarks
echo ""

echo -e "${GREEN}✓ Benchmarks complete!${NC}"
echo "Results: target/criterion/report/index.html"

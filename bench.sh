#!/bin/bash
set -e

# Check if --parallel flag is provided
PARALLEL=false
if [[ "$1" == "--parallel" ]]; then
    PARALLEL=true
    echo "⚡ Running benchmarks in parallel mode (may affect accuracy)..."
else
    echo "Running benchmarks sequentially..."
fi

if [ "$PARALLEL" = true ]; then
    # Run each year's benchmarks in parallel
    echo "Starting parallel benchmark groups..."
    cargo bench --bench solutions --quiet 2025 &
    PID1=$!
    cargo bench --bench solutions --quiet 2024 &
    PID2=$!
    cargo bench --bench solutions --quiet 2023 &
    PID3=$!
    cargo bench --bench solutions --quiet 2022 &
    PID4=$!
    cargo bench --bench solutions --quiet 2021 &
    PID5=$!
    cargo bench --bench solutions --quiet 2015 &
    PID6=$!
    
    # Wait for all to complete
    wait $PID1 $PID2 $PID3 $PID4 $PID5 $PID6
    echo "All parallel benchmarks complete!"
else
    # Run sequentially (default - more accurate)
    cargo bench --quiet
fi

echo ""
echo "Updating README with benchmark results..."
cargo run --bin update_benchmarks --quiet

echo ""
echo "✅ Benchmarks complete and README updated!"

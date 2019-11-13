#!/bin/bash

while true; do
    cargo build && (
        ./target/debug/divsufsort
        vimdiff ./crosscheck/{rust,c}
    )
    echo "Press Enter to cycle..."
    read
done

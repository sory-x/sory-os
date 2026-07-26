#!/bin/bash
cd /home/sory/Bureau/soryos/cosmic-epoch/cosmic-sory-ia
CARGO_BUILD_JOBS=1 cargo build --release --jobs 1 2>&1
echo "EXIT=$?" >> build-result.log

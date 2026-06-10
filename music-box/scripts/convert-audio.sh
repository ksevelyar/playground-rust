#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:-bad-apple.mp3}"
OUTPUT="${2:-bad-apple-60.pcm}"

ffmpeg -y -i "$INPUT" \
    -f s16le \
    -ar 22050 \
    -ac 1 \
    -t 60 \
    "$OUTPUT"

echo "Wrote $(du -h "$OUTPUT" | cut -f1) to $OUTPUT"

#!/bin/bash
cargo run --release -- README.md 2> output || (reset && bat output)

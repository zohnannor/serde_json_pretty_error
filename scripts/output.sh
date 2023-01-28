#!/bin/sh
RUST_BACKTRACE=0 cargo run -q --example output |& ansi2html -ims solarized | head -n -2 > output.html

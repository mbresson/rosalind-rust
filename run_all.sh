#!/usr/bin/env bash

for bin in ./src/bin/*.rs; do
	cargo run --bin $(basename "${bin%.rs}")
done

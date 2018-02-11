
# rosalind-rust

This project contains my solutions to the problems from Rosalind platform (http://rosalind.info) to learn the basics of bioinformatics, and I also use it as an opportunity to practice Rust which I wanted to learn for a long time. As a result, the code may be much more verbose and over-engineered that it needs to. Any suggestion or correction to make my code more Rust-y is welcome.

## Test it

```bash
cargo test
```

## Run it

```bash
for bin in ./src/bin/*.rs; do
	cargo run --bin $(basename "${bin%.rs}")
done
```

Voilà.

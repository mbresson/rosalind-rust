
# rosalind-rust

This project contains my solutions to the problems from Rosalind platform (http://rosalind.info) to learn the basics of bioinformatics (with the help of this online course: https://www.canal-u.tv/producteurs/inria/bioinformatics_genomes_and_algorithms), and I also use it as an opportunity to practice Rust which I wanted to learn for a long time. As a result, the code may be much more verbose and over-engineered that it needs to. Any suggestion or correction to make my code more Rust-y is welcome.

## Test it

```bash
cargo test
```

## Run it

### Run a single file

```bash
cargo run --bin filename-without-rs
```

Example:

```bash
cargo run --bin finding-a-motif-in-dna
```

### Run them all

One Script to rule them all,
One Script to find them,
One Script to bring them all,
And in the unix-ness run them.

```bash
for bin in ./src/bin/*.rs; do
	cargo run --bin $(basename "${bin%.rs}")
done
```

Voilà.

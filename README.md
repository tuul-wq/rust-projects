# Rust-projects

A bundle of small Rust pet projects used as a learning playground.

This repository is organized as a single Cargo workspace, where each project lives in `crates/` and builds as its own executable.

## Purpose

- Practice Rust fundamentals incrementally
- Keep experiments and exercises in one place
- Learn by building many small focused programs

## Structure

- `crates/guessing-game`: number guessing CLI
- `crates/grade-analyzer`: loops/conditions/arrays practice
- `crates/ownership-lab`: ownership and borrowing practice

## Run

From the repository root:

```bash
cargo run -p guessing-game
cargo run -p grade-analyzer
cargo run -p ownership-lab
```

## Check all projects

```bash
cargo check --workspace
```

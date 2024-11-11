# Learn Rust

Welcome to **Learn Rust**! This repository is designed to help me as Go developers learn Rust by comparing familiar concepts from Go and exploring Rust's unique features.

## Table of Contents
- [Introduction to Rust](#introduction-to-rust)
- [Setup](#setup)
- [Creating a New Package](#creating-a-new-package)
- [Running a Package](#running-a-package)
- [Content](#content)
  - [Hello World](./hello-world/)
  - [Variables](./variables/)
  - [Data Types](./data-types/)
  - [Operators](./operators/)
  - [Control Flows](./control-flows/)
  - [Collections](./collections/)
  - [Functions](./functions/)
  - And More Coming Soon!

### Introduction to Rust
Rust is a systems programming language focused on speed, memory safety, and parallelism. With Rust’s ownership model and type system, it brings unique advantages and challenges for developers coming from Go. This guide will introduce Rust concepts by leveraging the knowledge you already have from Go.

### Setup
To get started with Rust, [install Rust](https://www.rust-lang.org/tools/install) by following the official installation guide.

```bash
# Install Rust and Cargo
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installing Rust, you can verify the installation by checking the version of Rust and Cargo.

```bash
# Check Rust version
$ rustc --version
```

### Creating a New Package
```bash
# Create a new package
$ cargo new package-name
```

### Running a Package
```bash
# Change directory to the package
$ cd package-name
$ cargo run
```



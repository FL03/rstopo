# rstopo

[![crates.io](https://img.shields.io/crates/v/concision.svg)](https://crates.io/crates/rstopo)
[![docs.rs](https://docs.rs/concision/badge.svg)](https://docs.rs/rstopo)

[![clippy](https://github.com/FL03/rstopo/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/rstopo/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/rstopo/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/rstopo/actions/workflows/rust.yml)

***

### _The library is currently in the early stages of development and is not yet ready for production use._

rstopo is a mathematical library for Rust with a focus on topology. It is designed to be a lightweight and flexible library for machine learning and scientific computing.

## Features

* **Linear Models**: Linear regression, logistic regression, and linear classifiers.

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/rstopo.git
cd rstopo
```

```bash
cargo build --features full -r --workspace
```

## Usage

### Example: Linear Model (biased)

```rust
    extern crate rstopo;

    fn main() -> anyhow::Result<()> {
        tracing_subscriber::fmt::init();
        tracing::info!("Starting linear model example");


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

* [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
* [MIT](https://choosealicense.com/licenses/mit/)

# I Am Rust

A comprehensive collection of Rust programming language examples and concepts for learning purposes. This repository contains code examples that demonstrate the fundamentals of Rust programming language, organized by concept.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Running Examples](#running-examples)
- [Contributing](#contributing)
- [License](#license)

## Installation

### Prerequisites

- Rust and Cargo (install from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install))

### Clone the Repository

```bash
git clone https://github.com/yourusername/i_am_rust.git
cd i_am_rust
```

## Usage

This repository is organized into modules, each focusing on a specific Rust concept. You can:

1. Build all examples:
```bash
make build
```

2. Run a specific example:
```bash
make <example-name>
```

For example, to run the "tuples" example:
```bash
make tuples
```

3. Clean the output directory:
```bash
make clean
```

## Project Structure

The project is organized into the following directories, each containing examples for specific Rust concepts:

- **formatted-print**: Examples of formatted printing in Rust
- **primitives**: Examples of Rust primitive types (tuples, operators, arrays)
- **custom-types**: Examples of custom types in Rust (structures, enums, constants)
- **variable-bindings**: Examples of variable bindings (mutability, scope and shadowing)
- **types**: Examples of type-related concepts (casting, literals, inference, aliasing)
- **conversion**: Examples of type conversion
- **flow-of-control**: Examples of control flow (if-else, loop, match, if-let, functions, closures)
- **functions**: Examples of function-related concepts (higher order functions, diverging functions)
- **modules**: Examples of module-related concepts (visibility)
- **generics**: Examples of generic programming (functions, traits, bounds, new type idiom, associated items)
- **scoping-rules**: Examples of scoping rules (ownership and moves, borrowing, the ref pattern, lifetimes)

## Running Examples

Here are some examples of how to run specific modules:

```bash
# Run formatted print example
make run

# Run tuples example
make tuples

# Run operators example
make operators

# Run arrays example
make arrays

# Run structures example
make structures

# Run ownership and moves example
make ownership_and_moves

# Run borrowing example
make borrowing

# Run lifetimes example
make lifetimes
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).

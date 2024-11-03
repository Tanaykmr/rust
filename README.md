# Rust Learning Repository

This repository is a personal journey to learn and practice Rust, a systems programming language that emphasizes safety, speed, and concurrency. Each file in the `src/bin` directory focuses on different core Rust concepts, covering everything from basic types and control flow to advanced concepts like borrowing, vectors, iterators, and traits.

## Repository Structure

``` bash
src
├── bin
│   ├── 1-is_even.rs              # Check if a number is even
│   ├── 2-fib.rs                  # Generate Fibonacci sequence
│   ├── 3-get-string-length.rs    # Get length of a string
│   ├── 4-struct-user.rs          # Define and use a user struct
│   ├── 5-struct-rect-impl.rs     # Struct with methods (Rectangle)
│   ├── 6-enums.rs                # Working with enums
│   ├── 7-option.rs               # Using Option for null values
│   ├── 8-result.rs               # Handling Result for error handling
│   ├── 9-package-management.rs   # Package management in Rust
│   ├── 10-mutability.rs          # Exploring mutability concepts
│   ├── 11-moving1.rs             # Understanding data moving in Rust
│   ├── 12-moving1.rs             # Further moving exercises
│   ├── 13-borrowing.rs           # Borrowing data in Rust
│   ├── 14-borrowing-and-editing.rs # Mutably borrowing data and editing
│   ├── 15-vectors.rs             # Working with vectors
│   ├── 16-take-vector-input.rs   # Taking input to fill a vector
│   ├── 17-hashmaps.rs            # Introduction to hashmaps
│   ├── 18-hashmap-q1.rs          # Practical exercises with hashmaps
│   ├── 19-iterators.rs           # Understanding iterators in Rust
│   ├── 20-consumer-adaptor.rs    # Using consumer adaptors with iterators
│   ├── 21-iterator-adaptor.rs    # Using iterator adaptors
│   ├── 22-iterator-q1.rs         # Iterator exercises
│   ├── 23-strings.rs             # Working with strings in Rust
│   ├── 24-strings-q1.rs          # String manipulation exercises
│   ├── 25-traits.rs              # Using and defining traits
└── main.rs                       # Main entry point for Rust exercises
```

## Key Concepts Covered

	•	Basic Programs: Programs like checking if a number is even, generating Fibonacci sequences, and working with strings.
	•	Structs and Enums: Understanding how to create and use structs and enums, essential building blocks for Rust programs.
	•	Option and Result: Handling optional values and error management, a crucial part of Rust’s safety features.
	•	Data Ownership: Concepts of moving, borrowing, and mutability, which are foundational for memory safety.
	•	Collections: Working with vectors and hashmaps, Rust’s primary collection types.
	•	Iterators: Using iterators for efficient data processing, including consumer and iterator adaptors.
	•	Strings and Traits: Managing strings and defining traits to achieve polymorphism.

---

## Getting Started

To run any of the exercises, navigate to the src/bin folder and use the following command:

`cargo run --bin <filename>`

Replace <filename> with the name of the file you want to run (without the .rs extension).

## Requirements

- **Rust**: Make sure to have the latest version of Rust installed. You can install it via [rustup](https://www.rust-lang.org/tools/install).

## Contributing

This repository is for self-learning purposes. However, feel free to open issues or suggest improvements if you find any code enhancements.

_Happy coding!_

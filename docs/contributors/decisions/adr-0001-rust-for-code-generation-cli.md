# **ADR-0001** Implement GOATS Code Generation CLI in Rust

**Author**: Pierre Fouilloux

![Accepted](https://img.shields.io/badge/status-accepted-success) ![Date](https://img.shields.io/badge/Date-27_Jul_2023-lightblue)

## Context and Problem Statement

The GOATS project requires a command-line tool to generate client-side serialization/deserialization logic from server-side code. We need to decide on the programming language and technology stack for implementing this CLI tool. The chosen technology should align with the project's goals of being "good enough" for production while avoiding unnecessary complexity.

## Decision Drivers

* Performance requirements for parsing and generating code
* Cross-platform compatibility (Linux, macOS, Windows)
* Developer experience and maintenance burden
* Safety and correctness, especially for type handling
* Ease of distribution to end users
* Ecosystem support for parsing and code generation

## Considered Options

* Rust
* Go
* Python
* TypeScript/Node.js
* Java/Kotlin

## Decision Outcome

Chosen option: "Rust", because it provides the best balance of performance, safety, and appropriate features for a code generation tool focused on type serialization. Its strong type system and memory safety features align well with the requirements of correctly handling various data types and generating serialization code.

### Consequences

* Good, because Rust's performance will allow the tool to efficiently process large codebases
* Good, because memory safety guarantees help prevent bugs in the code generation logic
* Good, because the resulting CLI tool will be a single binary with no runtime dependencies
* Good, because cross-compilation support makes it easy to distribute for different platforms
* Good, because libraries like syn, quote, and proc-macro provide solid foundations for code manipulation
* Bad, because Rust has a steeper learning curve that may slow initial development

### Confirmation

We will confirm this decision is appropriate by:

1. Building a prototype that demonstrates the feasibility of implementing the core parsing and generation features in Rust
2. Measuring the performance and memory usage to ensure they meet our requirements

## Pros and Cons of the Options

### Rust

* Good, because it offers exceptional performance for parsing and code generation
* Good, because its strong type system is well-suited for a type-focused tool
* Good, because it produces standalone binaries that are easy to distribute
* Good, because it has growing ecosystem support for parsing and AST manipulation
* Good, because memory safety guarantees help prevent bugs without garbage collection overhead
* Neutral, because while compilation is strict, it catches many errors at compile time
* Bad, because it has a steeper learning curve compared to some alternatives
* Bad, because development might be slower initially due to fighting with the borrow checker

### Go

* Good, because it's simpler to learn than Rust
* Good, because it has good performance characteristics
* Good, because it produces standalone binaries
* Good, because it has a straightforward concurrency model
* Neutral, because its type system is adequate but less powerful than Rust's
* Bad, because it lacks some of the memory safety guarantees of Rust
* Bad, because the ecosystem for AST manipulation and code generation is less mature

### Python

* Good, because it enables rapid development with less boilerplate
* Good, because it has a vast ecosystem with many libraries
* Good, because most developers are already familiar with it
* Neutral, because dynamic typing might be less suitable for a type-focused tool
* Bad, because performance may be insufficient for large codebases
* Bad, because distribution requires managing runtime dependencies

### TypeScript/Node.js

* Good, because it would be familiar for web developers
* Good, because it has excellent JSON handling capabilities
* Good, because TypeScript's type system is quite advanced
* Neutral, because performance is adequate for many use cases
* Bad, because distribution and dependency management can be complex
* Bad, because it's less suitable for a standalone CLI tool

### Java/Kotlin

* Good, because they have mature ecosystems and tooling
* Good, because Kotlin offers modern language features with good safety
* Good, because many developers are familiar with the JVM ecosystem
* Neutral, because performance is generally good with some overhead
* Bad, because distribution requires bundling a JVM or requiring users to have one
* Bad, because the resulting tool would be heavier than alternatives

## More Information

The decision to use Rust aligns with the project's goal of creating a tool that is "good enough" for production use while avoiding unnecessary complexity. While Rust itself has a learning curve, the benefits of its performance, safety, and type system make it a strong choice for implementing a code generation tool focused on type serialization.

We should revisit this decision if:

1. We encounter significant development barriers that slow progress considerably
2. The ecosystem support proves insufficient for our needs
3. Team composition changes dramatically, affecting our Rust expertise

# **ADR-0002** Use Annotations for Serialization Schema Definition

**Author**: Pierre Fouilloux

![Accepted](https://img.shields.io/badge/status-accepted-success) ![Date](https://img.shields.io/badge/Date-03_Aug_2023-lightblue)

## Context and Problem Statement

The GOATS project requires a mechanism for defining which types should be serialized and how they should be represented in the serialized format. There are multiple established approaches to defining serialization schemas:

1. Using a dedicated Interface Definition Language (IDL) or Domain-Specific Language (DSL)
2. Using annotations/attributes/decorators in the source language
3. Using reflection or conventional patterns without explicit markup

We need to decide which approach will best serve the project's goal of being "good enough to use in production, while eschewing the complexity introduced by more performant frameworks."

## Decision Drivers

* Developer experience and ease of adoption
* Flexibility and expressiveness for different use cases
* Maintainability and evolution of schemas over time
* Integration with existing development workflows and tools
* Compatibility with the project's "Generally Okay" philosophy
* Support for the code generation approach established in ADR-0001

## Considered Options

* Dedicated IDL/DSL (similar to Protocol Buffers, Thrift)
* Annotations/attributes/decorators in source code
* Pure reflection-based approach (no explicit schema)
* Convention-based approach (following structural patterns)

## Decision Outcome

Chosen option: "Annotations/attributes/decorators in source code", because this approach provides the best balance of explicit schema definition and development simplicity. It integrates smoothly with existing codebases and development workflows, making it easier for developers to adopt GOATS without significant changes to their practices.

### Consequences

* Good, because developers can define serialization requirements directly in their domain models
* Good, because it eliminates the need to learn a separate schema language
* Good, because changes to models and their serialization behavior can evolve together
* Good, because it provides IDE support including code completion and refactoring tools
* Good, because it supports the code generation approach established for the Rust CLI
* Bad, because it couples the serialization logic more closely to the programming language
* Bad, because it may add visual noise to domain models if overused
* Bad, because it could be harder to generate standalone documentation

### Confirmation

We will confirm this decision is appropriate by:

1. Implementing prototype annotations in several common server-side languages
2. Testing the expressiveness by implementing complex real-world models
3. Getting feedback from developers regarding the usability and clarity
4. Ensuring the annotation approach can be effectively processed by the Rust CLI tool

## Pros and Cons of the Options

### Annotations/attributes/decorators in source code

* Good, because they work within the native language ecosystem
* Good, because they reduce cognitive overhead by avoiding context switching
* Good, because they integrate with existing IDE tooling (code completion, navigation)
* Good, because they eliminate the need for a separate compilation/generation step during development
* Good, because they co-evolve with the code they describe
* Neutral, because expressiveness varies by host language capabilities
* Bad, because they're inherently language-specific
* Bad, because they can create visual noise in the codebase

### Dedicated IDL/DSL (similar to Protocol Buffers, Thrift)

* Good, because it's language-agnostic and enables cross-language serialization
* Good, because it provides a single source of truth for the schema
* Good, because it can be more expressive for specialized serialization needs
* Good, because it separates domain model from serialization concerns
* Neutral, because tooling support varies by language ecosystem
* Bad, because it introduces a new language for developers to learn
* Bad, because it adds complexity to the development workflow
* Bad, because it requires synchronization between schema and implementation

### Pure reflection-based approach

* Good, because it requires minimal developer effort
* Good, because domain models remain clean with no serialization-specific markup
* Good, because it's adaptable to changing model structures
* Neutral, because reflection capabilities vary significantly between languages
* Bad, because it offers limited control over serialization behavior
* Bad, because it may have performance implications
* Bad, because implicit behavior may lead to unexpected serialization results

### Convention-based approach

* Good, because it reduces explicit markup in code
* Good, because it encourages consistent patterns
* Good, because models remain relatively clean
* Neutral, because it balances explicitness and implicitness
* Bad, because conventions may be harder to discover than explicit annotations
* Bad, because deviating from conventions may be complex
* Bad, because it may be less flexible for edge cases

## More Information

The annotation-based approach aligns directly with the GOATS philosophy of being "Generally Okay" - prioritizing developer experience and practical utility over maximum theoretical purity or performance. While annotations do introduce some language specificity, this is a reasonable trade-off given the significant usability benefits.

We should revisit this decision if:

1. Developer feedback indicates annotations are too restrictive
2. Supporting multiple languages proves more challenging than anticipated
3. We discover use cases that cannot be adequately expressed through annotations

This decision builds upon [ADR-0001](adr-0001-rust-for-code-generation-cli.md) (Using Rust for the CLI tool), as the CLI will need to parse and process these annotations from the source code to generate client serialization logic.

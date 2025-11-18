# ADR 0001: Core Engine Implementation Language and Architecture

## Status

Accepted

## Context

We need to develop a CLI tool that generates documentation and data flow diagrams from domain models.

Requirements:

- Need to support multiple programming languages (with different syntaxes) in the future
- Want to publish to npm first, but the implementation language doesn't need to be Node.js
- Want to publish to various language package repositories (npm, PyPI, crates.io, etc.)
- Main operations include domain model parsing, file I/O, text processing, and Mermaid format DFD generation

## Decision

### Architecture Pattern

Adopt the **Binary + Adapter Pattern**:

1. **Core Engine (Binary)**
   - Domain model parsing logic
   - DFD/Mermaid generation logic
   - Implementation language: **Rust**
   - Cross-compiled for each platform (Windows/macOS/Linux, x64/ARM)

2. **Language-specific Adapters**
   - Published to each language's package repository
   - Thin wrappers that invoke the binary
   - Responsible for reading configuration files, executing the binary, and processing results

### Implementation Language: Rust

Reasons for choosing Rust:

1. **Performance**: Fast processing suitable for parsing large codebases
2. **Memory Safety**: Reduces bugs and improves maintainability
3. **Cross-compilation**: Easy to build for multiple platforms with `cargo build --target`
4. **Ecosystem**: Rich ecosystem with parsers (`nom`, `pest`) and CLI frameworks (`clap`)
5. **Binary Size**: Can be optimized to an appropriate size

### Package Structure (for npm)

- `@dojigiri/core`: Rust binary (for each platform)
- `dojigiri`: Node.js adapter (thin wrapper that invokes the binary)

## Consequences

### Benefits

1. **Maintainability**: Core logic is centralized, making bug fixes and feature additions easier
2. **Performance**: Parsing and generation can be optimized in a single language
3. **Extensibility**: Supporting new languages can be achieved by simply adding a thin adapter
4. **Cross-platform**: Binary distribution enables support for each platform
5. **Language Independence**: Core engine is not dependent on any specific language

### Drawbacks and Considerations

1. **Binary Size**: Impact on npm package size (can be mitigated with optimization)
2. **Build and Distribution**: Need a mechanism for building and distributing for multiple platforms
3. **Version Management**: Need to synchronize versions between core engine and adapters
4. **Learning Curve**: Rust learning curve (though only the core engine needs Rust, adapters can be implemented in each language)

### Alternatives Considered

- **Go**: Easy cross-compilation, but inferior to Rust in terms of performance and binary size
- **Zig**: Excellent cross-compilation, but ecosystem is immature
- **Node.js**: Easy to package for npm, but has challenges with performance and future multi-language support

## Date

- 2025/11/18 crated

# ADR 0002: Annotation-based DFD Generation with Convention Validation

## Status

Accepted

## Context

To generate DFD (Data Flow Diagram) from domain models, we need to decide how to extract the necessary information. There are at least two approaches:

### Approach A: Parse Application Layer for Automatic Generation

- Automatically analyze the codebase (including application layer) to extract DFD structure
- Generate DFD from code analysis
- Pros: No manual annotation required, always up-to-date with code
- Cons: Complex parsing logic, may include unintended structures, design intent may not be reflected

### Approach B: Annotation-based with Convention Validation

- Define DFD structure explicitly through annotations on domain models
- Annotations serve as conventions/specifications
- Validate that application layer follows the conventions defined in annotations
- Fail the build if conventions are violated
- Pros: Explicit design intent, guarantees consistency between code and documentation, early detection of violations, aligns with Domain-Driven Design (DDD) principles
- Cons: Requires writing annotations, need to implement convention validation logic

## Decision

We adopt **Approach B: Annotation-based DFD Generation with Convention Validation**.

### Key Components

1. **Annotation System**
   - Annotations are added to domain models using language-appropriate comments
   - Annotations define:
     - Entity types (Process, DataStore, ExternalEntity)
     - Data flows between entities
     - Display names (can override default model names)

2. **Convention Validation**
   - Validate that application layer code follows the conventions defined in annotations
   - Check for:
     - Existence of declared data flows
     - Proper usage of entities
     - Adherence to defined relationships
   - Build fails if conventions are violated

3. **DFD Generation**
   - Generate Mermaid format DFD from annotations
   - Annotations serve as the single source of truth for the diagram structure

### Annotation Format (to be defined)

Annotations will be added using language-appropriate comment syntax. The exact format will be defined in a separate ADR or specification document.

Example (conceptual):

```rust
// @dojigiri:entity User
// @dojigiri:type DataStore
// @dojigiri:display "User Repository"
pub struct User { ... }

// @dojigiri:process CreateUser
// @dojigiri:flow User -> CreateUser: user_data
// @dojigiri:flow CreateUser -> User: saved_user
pub fn create_user(...) { ... }
```

## Consequences

### Benefits

1. **Explicit Design Intent**: Annotations make the intended DFD structure explicit and visible in the code
2. **Documentation as Code**: DFD structure is defined alongside the code, reducing drift
3. **Convention Enforcement**: Build-time validation ensures code follows the defined conventions
4. **DDD Alignment**: Supports Domain-Driven Design principles by making domain boundaries explicit
5. **Maintainability**: Changes to domain model require updating annotations, keeping documentation in sync

### Drawbacks and Considerations

1. **Annotation Overhead**: Developers need to write and maintain annotations
2. **Validation Logic Complexity**: Need to implement robust convention validation
3. **Language Support**: Annotation parsing needs to be implemented for each supported language
4. **Learning Curve**: Developers need to learn annotation syntax and conventions

### Alternatives Considered

- **Approach A (Automatic Parsing)**: Rejected because it doesn't capture design intent and may generate incorrect or unintended diagrams
- **Hybrid Approach**: Considered but rejected to keep the system simple and focused. Could be added in the future if needed.

## Date

2025-11-18

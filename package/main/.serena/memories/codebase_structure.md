# Codebase Structure

## Source Directory (/src/)

```tree
src/
├── index.ts                 # Main export file
├── types/                   # Shared type definitions ($/* alias)
├── Array/                   # Array manipulation functions
├── Math/                    # Mathematical functions
├── String/                  # String manipulation functions
├── Date/                    # Date manipulation functions
├── Validate/                # Validation functions
├── Crypto/                  # Cryptographic functions
├── Color/                   # Color conversion functions
├── IP/                      # IP manipulation functions
├── Object/                  # Object manipulation functions
├── Function/                # Function manipulation functions
├── Error/                   # Error handling functions
├── Graph/                   # Graph algorithms and data structures
├── Tool/                    # Tool functions
├── DataStructure/           # Data structures
├── UA/                      # User Agent functions
├── Unit/                    # Unit conversion functions
├── Time/                    # Time conversion functions
├── Simple/                  # Simple version functions
├── Advance/                 # Advanced functions
├── Consts/                  # Constant definitions
└── tests/                   # Test directory
    ├── unit/                # Unit tests (mirrors source structure)
    ├── integration/         # Integration tests
    └── benchmark/           # Performance tests
```

## Build Output

- `./module/` - TypeScript compilation output
- `./common-module/` - CommonJS version output

## Path Aliases

- `@/*` → `src/*` (internal imports)
- `$/*` → `src/types/*` (type imports)

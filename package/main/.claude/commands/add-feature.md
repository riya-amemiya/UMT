---
allowed-tools: Bash(*), Read(*), Write(*), Edit(*), MultiEdit(*), Glob(*), Grep(*), LS(*), Task(*)
description: Add new feature to UMT project with comprehensive workflow (tests, lint, build, README)
---

# UMT Feature Addition Workflow

Implement the new feature `$ARGUMENTS` following these strict requirements:

## Project Configuration

- Package info: @package.json
- README overview: @README.md

## Implementation Rules

1. **Test Implementation**: Create comprehensive tests with 100% coverage
2. **Lint Compliance**: `bun run lint` must pass
3. **Build Success**: `bun run build` must pass
4. **No External Dependencies**: Adding new dependencies is forbidden
5. **Doc Comments Required**: Write JSDoc for all public functions
6. **Runtime Agnostic**: No browser-only or Node.js-only implementations
7. **README Update**: Update README.md after implementation

## Implementation Steps

### Phase 1: Research & Design

1. Investigate existing codebase for similar functionality
2. Determine appropriate module placement
3. Consider type definitions in `src/types/` if needed

### Phase 2: Implementation

1. Implement main functionality in appropriate module directory
2. Add type definitions if necessary
3. Export from module's `index.ts`
4. Export from main `src/index.ts`

### Phase 3: Testing

1. Create unit tests in `src/tests/unit/`
2. Include comprehensive edge cases and error scenarios
3. Verify `bun run test` passes completely

### Phase 4: Quality Assurance

1. Run and fix `bun run lint`
2. Run and verify `bun run build`
3. Validate generated module files

### Phase 5: Documentation

1. Add new feature to appropriate README.md section
2. Include usage examples and API documentation

## Code Quality Requirements

- **Naming**: camelCase or PascalCase
- **Import Order**: builtin → external → internal → parent → sibling → index
- **Indentation**: 2 spaces
- **Line Width**: 80 characters
- **Array Types**: Use `T[]` format
- **Error Handling**: Provide descriptive error messages

Begin implementing feature `$ARGUMENTS` now.

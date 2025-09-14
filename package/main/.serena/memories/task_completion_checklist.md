# Task Completion Checklist

## Required Execution Items

### 1: Lint and Type Check

- `bun run lint` - Run ESLint + Biome + TypeScript type checking
  - Verify no ESLint errors
  - Verify no Biome format errors
  - Verify no TypeScript type errors

### 2: Test Execution

- `bun run test` - Run all tests
  - Verify existing tests pass
  - Verify tests are added for new features

### 3: Build Verification

- `bun run build` - Verify successful build
  - Verify no compilation errors
  - Verify path aliases are correctly resolved

## Important Notes

- **Use Bun** - Execute commands with Bun, not Node.js
- **No commits** - Only commit when explicitly requested by user
- **Zero dependency principle** - Check existing usage before adding new libraries
- **Type safety** - Maintain strict TypeScript settings

## Code Quality Verification

- Follow existing code conventions
- Comply with security best practices
- Add appropriate type definitions

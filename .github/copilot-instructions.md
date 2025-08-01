# UMT Project Guidelines for Claude

## Build/Test/Lint Commands

- Build: `bun run build`
- Lint: `bun run lint` (ESLint + Biome + TypeScript)
- Test all: `bun run test`
- Test single file: `bun run test src/tests/unit/path/to/test.test.ts`
- Format: `bun run format` (uses Biome)

## Code Style Guidelines

- **File naming**: `camelCase` or `PascalCase` (enforced by ESLint)
- **Imports**: Ordered by builtin → external → internal → parent → sibling → index
- **Formatting**:
  - Indentation: 2 spaces
  - Line width: 80 characters
  - No default exports (use named exports)
- **TypeScript**:
  - Strict typing preferred
  - Avoid explicit `any` unless necessary
  - Use consistent array type notation
- **Error handling**:
  - Use error messages
  - Throw only Error objects
- **Documentation**: Include JSDoc comments for public functions
- **Testing**: Write unit tests for new functionality

## Notes

- Project uses ESLint, Biome, and TypeScript for code quality
- Import order is enforced with separate groups and alphabetical sorting
- Ensure all commands are run using `bun`
- Follow the code style guidelines outlined above

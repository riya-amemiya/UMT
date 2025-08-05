# Code Style and Conventions

## File Naming

- `camelCase` or `PascalCase` (enforced by ESLint unicorn plugin)

## Import Conventions

- Order: builtin → external → internal → parent → sibling → index (auto-sorted by ESLint)
- No default exports (enforced by Biome `noDefaultExport`)

## Formatting Conventions

- Indentation: 2 spaces
- Line width: 80 characters
- Array type notation: `T[]` preferred over `Array<T>`

## TypeScript Configuration

- Strict mode enabled (all strict flags)
- `noUnusedLocals`, `noUnusedParameters` enabled
- Path aliases resolved with `tsc-alias` after compilation

## Error Handling

- Use descriptive error messages (Biome `useErrorMessage`)
- Throw only Error objects (Biome `useThrowOnlyError`)

## Dependencies

- Main package is dependency-free (dev dependencies only)
- Must check if library/framework is already used in codebase before using

## Security

- Never expose or log secrets and keys
- Never commit secrets or keys to repository

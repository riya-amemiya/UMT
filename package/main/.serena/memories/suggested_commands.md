# Suggested Commands

## Build Related

- `bun run build` - TypeScript compilation (includes path alias resolution)
- `bun run build:clean` - Build after clearing module directory
- `bun run build:full` - Build in all formats (ESM, CJS, Babel)

## Development Tools

- `bun run lint` - ESLint + Biome + TypeScript type checking
- `bun run format` - Biome formatter
- `bun run test` - Run Jest tests
- `bun run test src/tests/unit/path/to/test.test.ts` - Single file test

## Type Checking

- `tsc` - Run TypeScript type checking only

## JavaScript Execution

- `bun run <fileName.js>` - Run JavaScript files directly with Bun

## CI Usage

- `bun run lint:ci` - Linting for CI environment (no fixes)

## Important Notes

- **Do not use Node.js** - This project uses Bun
- Path aliases require `tsc-alias`

# Type File Conventions

## Important: Type Definition Structure

Types should be organized as follows:

- Create a directory under `/src/types/` with lowercase naming (e.g., `graph`, not `Graph`)
- Each type/interface should be in its own file with camelCase naming
- One type/interface per file
- Create an `index.ts` file that re-exports all types

### Example Structure

```tree
src/types/graph/
├── graph.ts              # export interface Graph<T> { ... }
├── graphOptions.ts       # export interface GraphOptions { ... }
├── pathResult.ts         # export interface PathResult<T> { ... }
├── edge.ts              # export interface Edge<T> { ... }
├── heuristicFunction.ts  # export type HeuristicFunction<T> = ...
└── index.ts             # Re-exports all types
```

### Naming Conventions

- Directory: lowercase (e.g., `graph`)
- Type files: camelCase (e.g., `graphOptions.ts`)
- Interfaces/Types: PascalCase (e.g., `GraphOptions`)

This pattern ensures consistency and makes types easy to find and import.

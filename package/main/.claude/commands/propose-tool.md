---
allowed-tools: Read(*), Glob(*), Grep(*), LS(*), Task(*), WebSearch(*)
description: Propose new utility tools for UMT project based on gap analysis and modern best practices
---

# UMT Tool Proposal Generator

Analyze the current UMT toolkit and propose new utility functions for category `$ARGUMENTS` based on:

## Current Project Analysis

- Package info: @package.json
- README overview: @README.md
- Main exports: @src/index.ts

## Proposal Methodology

### 1. Gap Analysis

- Review existing functions in the specified category
- Identify common JavaScript/TypeScript patterns not covered
- Analyze popular utility libraries for inspiration (lodash, ramda, date-fns, etc.)

### 2. Modern Standards Alignment

- Consider ES2024+ features and patterns
- Ensure TypeScript-first design with strong typing
- Focus on functional programming principles where applicable

### 3. UMT Compatibility Check

- Zero external dependencies requirement
- Runtime-agnostic implementation
- Follows existing code style and patterns
- Maintains consistent API design

## Proposal Format

For each proposed tool, provide:

### Function Signature

```typescript
export function toolName(params: Types): ReturnType;
```

### Purpose & Use Cases

- Clear description of what the function does
- 2-3 practical use cases
- Comparison with existing alternatives (if any)

### Implementation Strategy

- High-level approach without full code
- Key algorithms or techniques to use
- Type safety considerations

### Testing Strategy

- Edge cases to cover
- Performance considerations
- Expected test coverage areas

## Categories to Consider

- **Array**: sorting, grouping, transformation, statistical operations
- **Object**: manipulation, transformation, validation, deep operations
- **String**: parsing, formatting, validation, transformation
- **Math**: calculations, conversions, statistical functions
- **Date**: formatting, manipulation, calculation, timezone handling
- **Function**: composition, memoization, debouncing, currying
- **Type**: guards, assertions, utilities, validation
- **Async**: promise utilities, async iteration, concurrency control

## Output Requirements

1. Propose 3-5 new utility functions for category `$ARGUMENTS`
2. Ensure each proposal addresses a real development need
3. Verify no overlap with existing UMT functions
4. Consider implementation complexity vs utility value
5. Prioritize proposals by impact and usefulness

Begin analysis and proposal generation for category: `$ARGUMENTS`

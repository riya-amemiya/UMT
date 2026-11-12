import type { Result } from "@/Error/safeExecute";

/**
Extracts the value from a successful Result, or returns the provided
default value if the Result is an error.

@param result - The Result to unwrap.
@param defaultValue - The value returned when the Result is an error.
@returns The success value, or the default value for an error Result.

@example
```typescript
const success = { type: "success", value: 5 } as const;
unwrapOr(success, 0);
// 5

const error = { type: "error", error: new Error("fail") } as const;
unwrapOr(error, 0);
// 0
```
*/
export const unwrapOr = <V, E, D = V>(
  result: Result<V, E>,
  defaultValue: D,
): V | D => {
  return result.type === "success" ? result.value : defaultValue;
};

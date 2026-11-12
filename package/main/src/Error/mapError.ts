import { errorFunction, type Result } from "@/Error/safeExecute";

/**
Transforms the error inside a failed Result using the provided
mapping function. If the Result is a success, it is returned unchanged.

@param result - The Result to transform.
@param function_ - The function to apply to the error.
@returns A new Result with the transformed error, or the original success.

@example
```typescript
const error = { type: "error", error: "not found" } as const;
mapError(error, (message) => new Error(message));
// { type: "error", error: Error("not found") }

const success = { type: "success", value: 5 } as const;
mapError(success, (message: string) => new Error(message));
// { type: "success", value: 5 }
```
*/
export const mapError = <V, E, F>(
  result: Result<V, E>,
  function_: (error: E) => F,
): Result<V, F> => {
  return result.type === "error"
    ? errorFunction(function_(result.error))
    : result;
};

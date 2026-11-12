/**
Serializes a key-value record into a query string.

The inverse of `parseQueryString`. Keys and values are encoded as
`application/x-www-form-urlencoded`, the same way `buildUrl` appends
parameters. The result has no leading "?".

@param parameters - An object of key-value pairs to serialize
@returns The query string, or an empty string for an empty record

@example
```typescript
stringifyQuery({ page: "1", q: "search" });
// "page=1&q=search"

stringifyQuery({ q: "hello world", tag: "a&b" });
// "q=hello+world&tag=a%26b"
```
*/
export const stringifyQuery = (parameters: Record<string, string>): string =>
  new URLSearchParams(parameters).toString();

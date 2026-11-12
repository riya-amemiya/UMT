const byteUnits = ["B", "KB", "MB", "GB", "TB", "PB"];

/**
Formats a byte count as a human-readable string using base 1024.
The unit is the largest of B, KB, MB, GB, TB and PB that does not exceed the
absolute value (values of a PB or more stay in PB). The value is rounded to
`decimals` places with `Math.round` semantics and printed without trailing
zeros.

@param bytes - The number of bytes (may be negative or fractional)
@param decimals - The number of decimal places (default 2)
@returns The formatted size, such as "1.5 KB"

@example
```typescript
formatBytes(0);              // "0 B"
formatBytes(1024);           // "1 KB"
formatBytes(1536);           // "1.5 KB"
formatBytes(1_234_567);      // "1.18 MB"
formatBytes(1_234_567, 0);   // "1 MB"
formatBytes(-2048);          // "-2 KB"
```
*/
export const formatBytes = (bytes: number, decimals = 2): string => {
  const absolute = Math.abs(bytes);
  const index = Math.max(
    byteUnits.filter((_, unitIndex) => absolute >= 1024 ** unitIndex).length -
      1,
    0,
  );
  const factor = 10 ** decimals;
  const value = Math.round((bytes / 1024 ** index) * factor) / factor;
  return `${value} ${byteUnits[index]}`;
};

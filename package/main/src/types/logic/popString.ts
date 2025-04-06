// Type to remove last character from string
export type PopString<S extends string> = S extends `${infer R}${string}`
  ? R
  : never;

// Type to remove the first character from a string
export type ShiftString<S extends string> = S extends `${string}${infer R}`
  ? R
  : never;

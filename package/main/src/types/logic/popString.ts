// Type to remove last character from string
export type PopString<S extends string> = S extends `${infer F}${infer R}`
  ? R extends ""
    ? ""
    : `${F}${PopString<R>}`
  : never;

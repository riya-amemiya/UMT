// Type to reverse a string
export type StringReverse<S extends string> = S extends `${infer F}${infer R}`
  ? `${StringReverse<R>}${F}`
  : "";

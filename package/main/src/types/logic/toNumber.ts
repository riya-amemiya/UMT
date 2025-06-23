// Type to convert to number
export type ToNumber<S> = S extends `${infer N extends number}` ? N : never;

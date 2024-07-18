// number型に変換する型
export type ToNumber<S> = S extends `${infer N extends number}` ? N : never;

type ZIP<T, O extends unknown[] = []> = T extends [infer Head, ...infer Tail]
  ? Head extends unknown[]
    ? ZIP<Tail, [...O, Head[number]]>
    : never
  : T extends []
    ? O[]
    : T;

export type ZipArrayType<T extends unknown[][]> = ZIP<T>;

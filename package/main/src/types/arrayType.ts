import { Length } from "./logicType";

type Chunk<
  T,
  N extends number,
  C extends unknown[] = [],
  R extends unknown[] = [],
> = T extends [infer F, ...infer L]
  ? Length<C> extends N
    ? Chunk<L, N, [F], [...R, C]>
    : Chunk<L, N, [...C, F], R>
  : Length<C> extends 0
  ? Length<R> extends 0
    ? T[]
    : R
  : [...R, C];

export type ChunkArrayType<T, N extends number> = Chunk<T, N>;

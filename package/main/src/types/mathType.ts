import { BGreaterThanA, Length } from "./logicType";

// 数値を配列に変換する
export type NumToArr<
  T extends number,
  U extends number[] = [],
> = U["length"] extends T ? U : NumToArr<T, [...U, U["length"]]>;

// たし算
export type Add<
  X extends number,
  Y extends number,
> = `${X}` extends `-${infer N extends number}`
  ? `${Y}` extends `-${infer M extends number}`
    ? `-${Add<N, M>}` extends `${infer R extends number}`
      ? R
      : never
    : Subtract<Y, N>
  : `${Y}` extends `-${infer M extends number}`
  ? Subtract<X, M>
  : Length<[...NumToArr<X>, ...NumToArr<Y>]>;

// 引き算
export type Subtract<
  A extends number,
  B extends number,
> = `${A}` extends `-${infer N extends number}`
  ? `${B}` extends `-${infer M extends number}`
    ? Subtract<M, N>
    : `-${Add<N, B>}` extends `${infer R extends number}`
    ? R
    : never
  : `${B}` extends `-${infer M extends number}`
  ? Length<[...NumToArr<A>, ...NumToArr<M>]>
  : NumToArr<A> extends [...NumToArr<B>, ...infer R]
  ? Length<R>
  : NumToArr<B> extends [...NumToArr<A>, ...infer R]
  ? `-${Length<R>}` extends `${infer M extends number}`
    ? M
    : never
  : never;

// 掛け算
export type Multiply<A extends number, B extends number> = MultiHelper<A, B>;

// 掛け算のヘルパー
type MultiHelper<
  X extends number,
  Y extends number,
  A extends number = 0,
> = `${X}` extends `-${infer N extends number}`
  ? `${Y}` extends `-${infer M extends number}`
    ? MultiHelper<N, M, A>
    : `-${MultiHelper<N, Y, A>}` extends `${infer R extends number}`
    ? R
    : never
  : `${Y}` extends `-${infer M extends number}`
  ? `-${MultiHelper<X, M, A>}` extends `${infer R extends number}`
    ? R
    : never
  : Y extends 0
  ? A
  : MultiHelper<X, Subtract<Y, 1>, Add<X, A>>;

// 割り算
export type Divide<A extends number, B extends number> = A extends 0
  ? never
  : B extends 0
  ? never
  : `${A}` extends `-${infer N extends number}`
  ? `${B}` extends `-${infer M extends number}`
    ? DivideHelper<N, M>
    : `-${DivideHelper<N, B>}` extends `${infer R extends number}`
    ? R
    : never
  : `${B}` extends `-${infer M extends number}`
  ? `-${DivideHelper<A, M>}` extends `${infer R extends number}`
    ? R
    : never
  : DivideHelper<A, B>;

// 割り算のヘルパー
type DivideHelper<
  X extends number,
  Y extends number,
  A extends number = 0,
> = BGreaterThanA<X, Y> extends true
  ? A
  : DivideHelper<Subtract<X, Y>, Y, Add<A, 1>>;

// 余り
export type Modulo<A extends number, B extends number> = BGreaterThanA<
  A,
  B
> extends true
  ? A
  : Modulo<Subtract<A, B>, B>;

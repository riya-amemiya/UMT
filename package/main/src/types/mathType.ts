import { BGreaterThanA, Length } from "./logicType";

// 数値を配列に変換する
export type NumToArr<
  T extends number,
  U extends number[] = [],
> = U["length"] extends T ? U : NumToArr<T, [...U, U["length"]]>;

// たし算
export type Add<X extends number, Y extends number> = Length<
  [...NumToArr<X>, ...NumToArr<Y>]
>;

// 引き算
export type Subtract<A extends number, B extends number> = NumToArr<A> extends [
  ...NumToArr<B>,
  ...infer R,
]
  ? Length<R>
  : 0;

// 掛け算
export type Multiply<A extends number, B extends number> = MultiHelper<A, B, 0>;

// 掛け算のヘルパー
type MultiHelper<
  X extends number,
  Y extends number,
  A extends number,
> = Y extends 0 ? A : MultiHelper<X, Subtract<Y, 1>, Add<X, A>>;

// 割り算
export type Divide<A extends number, B extends number> = DivideHelper<A, B, 0>;

// 割り算のヘルパー
type DivideHelper<
  X extends number,
  Y extends number,
  A extends number,
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

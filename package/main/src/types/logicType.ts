export type isBoolean<X> = X extends number
  ? X extends 0
    ? false
    : true
  : X extends string
  ? X extends ""
    ? false
    : true
  : X extends boolean
  ? X
  : X extends undefined
  ? false
  : X extends null
  ? false
  : X extends object
  ? X extends unknown[]
    ? X extends []
      ? false
      : true
    : true
  : X extends (...args: unknown[]) => unknown
  ? false
  : true;
export type AND<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? true
    : false
  : false;

export type OR<X, Y> = isBoolean<X> extends true
  ? true
  : isBoolean<Y> extends true
  ? true
  : false;

export type XOR<X, Y> = isBoolean<X> extends true
  ? isBoolean<Y> extends true
    ? false
    : true
  : isBoolean<Y> extends true
  ? true
  : false;
export type NOT<X> = isBoolean<X> extends true ? false : true;
export type NAND<X, Y> = NOT<AND<X, Y>>;
export type NOR<X, Y> = NOT<OR<X, Y>>;
export type XNOR<X, Y> = NOT<XOR<X, Y>>;
export type IMPLY<X, Y> = NOT<X> extends true ? true : isBoolean<Y>;

export type StringToArray<
  S extends string,
  T extends unknown[] = [],
> = S extends `${infer F}${infer R}` ? StringToArray<R, [...T, F]> : T;

export type Length<T extends unknown[]> = T["length"];
export type LengthOfString<S extends string,> = StringToArray<S>["length"];

export type Shift<T extends unknown[]> = T extends [unknown, ...infer R]
  ? R
  : never;
export type Pop<T extends unknown[]> = T extends [...infer R, unknown]
  ? R
  : never;
export type ShiftString<S extends string> = S extends `${string}${infer R}`
  ? R
  : never;
export type StringToUnion<S extends string> = S extends `${infer F}${infer R}`
  ? F | StringToUnion<R>
  : never;

export type IsAny<T> = 0 extends 1 & T ? true : false;

export type StringReverse<S extends string> = S extends `${infer F}${infer R}`
  ? `${StringReverse<R>}${F}`
  : "";

export type First8Chars<
  S extends string,
  T extends unknown[] = [],
> = T["length"] extends 8
  ? T extends [
      infer C1,
      infer C2,
      infer C3,
      infer C4,
      infer C5,
      infer C6,
      infer C7,
      infer C8,
      ...unknown[],
    ]
    ? `${C1 & string}${C2 & string}${C3 & string}${C4 & string}${C5 &
        string}${C6 & string}${C7 & string}${C8 & string}`
    : never
  : S extends `${infer Head}${infer Rest}`
  ? First8Chars<Rest, [...T, Head]>
  : never;

type biynaryAddParser<
  A extends string,
  B extends string,
  C extends "0" | "1" = "0",
> = [A, B] extends [`${infer A1}${infer A2}`, `${infer B1}${infer B2}`]
  ? A1 extends "0"
    ? B1 extends "0"
      ? C extends "0"
        ? `${biynaryAddParser<A2, B2, "0">}0`
        : `${biynaryAddParser<A2, B2, "0">}1`
      : C extends "0"
      ? `${biynaryAddParser<A2, B2, "0">}1`
      : `${biynaryAddParser<A2, B2, "1">}0`
    : B1 extends "0"
    ? C extends "0"
      ? `${biynaryAddParser<A2, B2, "0">}1`
      : `${biynaryAddParser<A2, B2, "1">}0`
    : C extends "0"
    ? `${biynaryAddParser<A2, B2, "1">}0`
    : `${biynaryAddParser<A2, B2, "1">}1`
  : `${C}`;

export type biynaryAdd<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
  Y extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = StringReverse<
  First8Chars<
    StringReverse<biynaryAddParser<StringReverse<X>, StringReverse<Y>>>
  >
>;

type biynaryComplementParser<X extends string> =
  X extends `${infer L}${infer R}`
    ? L extends "0"
      ? `1${biynaryComplementParser<R>}`
      : `0${biynaryComplementParser<R>}`
    : "";

export type biynaryComplement<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = First8Chars<biynaryAdd<biynaryComplementParser<X>, "00000001">>;

type biynaryToDecimalParser<
  X extends string,
  C extends unknown[] = [""],
  A extends unknown[] = [""],
  FL extends boolean = false,
> = X extends `${infer F}${infer R}`
  ? LengthOfString<X> extends 8
    ? F extends "1"
      ? `-${biynaryToDecimalParser<StringReverse<R>, C, A, true>}`
      : biynaryToDecimalParser<StringReverse<R>, C, A, FL>
    : FL extends false
    ? F extends "1"
      ? biynaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
      : biynaryToDecimalParser<R, [...C, ...C], A, FL>
    : F extends "1"
    ? biynaryToDecimalParser<R, [...C, ...C], A, FL>
    : biynaryToDecimalParser<R, [...C, ...C], [...A, ...C], FL>
  : FL extends true
  ? Length<A>
  : Length<Shift<A>>;
export type biynaryToDecimal<
  X extends `${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}${0 | 1}`,
> = biynaryToDecimalParser<X>;

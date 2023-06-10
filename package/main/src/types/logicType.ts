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
  ? // rome-ignore lint/suspicious/noExplicitAny: <explanation>
    X extends any[]
    ? X extends []
      ? false
      : true
    : true
  : X extends Function
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

export type isBoolean<X> = X extends number
    ? X extends 0
        ? false
        : true
    : X extends string
    ? X extends ''
        ? false
        : true
    : X extends boolean
    ? X
    : X extends undefined
    ? false
    : X extends null
    ? false
    : X extends object
    ? X extends Array<any>
        ? X extends []
            ? false
            : true
        : true
    : X extends Function
    ? false
    : true;
export type AND<X extends boolean, Y extends boolean> = X extends true
    ? Y extends true
        ? true
        : false
    : false;
export type OR<X extends boolean, Y extends boolean> = X extends true
    ? true
    : Y extends true
    ? true
    : false;
export type XOR<X extends boolean, Y extends boolean> = X extends true
    ? Y extends true
        ? false
        : true
    : Y extends true
    ? true
    : false;

export type NOT<X extends boolean> = X extends true ? false : true;
export type NAND<X extends boolean, Y extends boolean> = NOT<
    AND<X, Y>
>;
export type NOR<X extends boolean, Y extends boolean> = NOT<OR<X, Y>>;
export type XNOR<X extends boolean, Y extends boolean> = NOT<
    XOR<X, Y>
>;

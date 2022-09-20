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

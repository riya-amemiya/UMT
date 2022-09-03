export type AND<X extends boolean, Y extends boolean> = X extends true
    ? Y extends true
        ? true
        : false
    : false;
export type XOR<X extends boolean, Y extends boolean> = X extends true
    ? Y extends true
        ? false
        : true
    : Y extends true
    ? true
    : false;
export type OR<X extends boolean, Y extends boolean> = X extends true
    ? true
    : Y extends true
    ? true
    : false;

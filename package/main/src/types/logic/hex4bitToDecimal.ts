// Type to convert 4-bit hexadecimal to decimal
export type Hex4bitToDecimal<X extends string> = X extends "A"
  ? "10"
  : X extends "B"
    ? "11"
    : X extends "C"
      ? "12"
      : X extends "D"
        ? "13"
        : X extends "E"
          ? "14"
          : X extends "F"
            ? "15"
            : X;

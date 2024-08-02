// 4bitの16進数を10進数に変換する型
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

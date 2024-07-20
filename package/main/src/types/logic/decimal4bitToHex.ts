// 4bitの10進数を16進数に変換する型
export type Decimal4bitToHex<X extends string> = X extends "10"
  ? "A"
  : X extends "11"
    ? "B"
    : X extends "12"
      ? "C"
      : X extends "13"
        ? "D"
        : X extends "14"
          ? "E"
          : X extends "15"
            ? "F"
            : X;
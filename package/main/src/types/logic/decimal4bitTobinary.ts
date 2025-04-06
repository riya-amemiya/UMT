// Type to convert 4-bit decimal to binary
export type Decimal4bitTobinary<X extends string> = X extends "0"
  ? "0000"
  : X extends "1"
    ? "0001"
    : X extends "2"
      ? "0010"
      : X extends "3"
        ? "0011"
        : X extends "4"
          ? "0100"
          : X extends "5"
            ? "0101"
            : X extends "6"
              ? "0110"
              : X extends "7"
                ? "0111"
                : X extends "8"
                  ? "1000"
                  : X extends "9"
                    ? "1001"
                    : X extends "10"
                      ? "1010"
                      : X extends "11"
                        ? "1011"
                        : X extends "12"
                          ? "1100"
                          : X extends "13"
                            ? "1101"
                            : X extends "14"
                              ? "1110"
                              : X extends "15"
                                ? "1111"
                                : never;

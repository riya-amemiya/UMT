// booleanに変換する型
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
            : X extends (...arguments_: unknown[]) => unknown
              ? false
              : true;

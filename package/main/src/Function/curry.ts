/**
 * Curries a function.
 *
 * @param func - The function to curry.
 * @returns The curried function.
 *
 * @example
 * // Example with a function that adds three numbers
 * const add = (a: number, b: number, c: number) => a + b + c;
 * const curriedAdd = curry(add);
 *
 * console.log(curriedAdd(1)(2)(3)); // Output: 6
 * console.log(curriedAdd(1, 2)(3)); // Output: 6
 * console.log(curriedAdd(1, 2, 3)); // Output: 6
 */
export function curry<Result>(function_: () => Result): () => Result;
export function curry<Argument1, Result>(
  function_: (argument1: Argument1) => Result,
): (argument1: Argument1) => Result;
export function curry<Argument1, Argument2, Result>(
  function_: (argument1: Argument1, argument2: Argument2) => Result,
): {
  (argument1: Argument1): (argument2: Argument2) => Result;
  (argument1: Argument1, argument2: Argument2): Result;
};
export function curry<Argument1, Argument2, Argument3, Result>(
  function_: (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
  ) => Result,
): {
  (
    argument1: Argument1,
  ): {
    (argument2: Argument2): (argument3: Argument3) => Result;
    (argument2: Argument2, argument3: Argument3): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
  ): (argument3: Argument3) => Result;
  (argument1: Argument1, argument2: Argument2, argument3: Argument3): Result;
};
export function curry<Argument1, Argument2, Argument3, Argument4, Result>(
  function_: (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
  ) => Result,
): {
  (
    argument1: Argument1,
  ): {
    (
      argument2: Argument2,
    ): {
      (argument3: Argument3): (argument4: Argument4) => Result;
      (argument3: Argument3, argument4: Argument4): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
    ): (argument4: Argument4) => Result;
    (argument2: Argument2, argument3: Argument3, argument4: Argument4): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
  ): {
    (argument3: Argument3): (argument4: Argument4) => Result;
    (argument3: Argument3, argument4: Argument4): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
  ): (argument4: Argument4) => Result;
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
  ): Result;
};
export function curry<
  Argument1,
  Argument2,
  Argument3,
  Argument4,
  Argument5,
  Result,
>(
  function_: (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
    argument5: Argument5,
  ) => Result,
): {
  (
    argument1: Argument1,
  ): {
    (
      argument2: Argument2,
    ): {
      (
        argument3: Argument3,
      ): {
        (argument4: Argument4): (argument5: Argument5) => Result;
        (argument4: Argument4, argument5: Argument5): Result;
      };
      (
        argument3: Argument3,
        argument4: Argument4,
      ): (argument5: Argument5) => Result;
      (
        argument3: Argument3,
        argument4: Argument4,
        argument5: Argument5,
      ): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
    ): {
      (argument4: Argument4): (argument5: Argument5) => Result;
      (argument4: Argument4, argument5: Argument5): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
      argument4: Argument4,
    ): (argument5: Argument5) => Result;
    (
      argument2: Argument2,
      argument3: Argument3,
      argument4: Argument4,
      argument5: Argument5,
    ): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
  ): {
    (
      argument3: Argument3,
    ): {
      (argument4: Argument4): (argument5: Argument5) => Result;
      (argument4: Argument4, argument5: Argument5): Result;
    };
    (
      argument3: Argument3,
      argument4: Argument4,
    ): (argument5: Argument5) => Result;
    (argument3: Argument3, argument4: Argument4, argument5: Argument5): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
  ): {
    (argument4: Argument4): (argument5: Argument5) => Result;
    (argument4: Argument4, argument5: Argument5): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
  ): (argument5: Argument5) => Result;
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
    argument5: Argument5,
  ): Result;
};
export function curry<
  Argument1,
  Argument2,
  Argument3,
  Argument4,
  Argument5,
  Argument6,
  Result,
>(
  function_: (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
    argument5: Argument5,
    argument6: Argument6,
  ) => Result,
): {
  (
    argument1: Argument1,
  ): {
    (
      argument2: Argument2,
    ): {
      (
        argument3: Argument3,
      ): {
        (
          argument4: Argument4,
        ): {
          (argument5: Argument5): (argument6: Argument6) => Result;
          (argument5: Argument5, argument6: Argument6): Result;
        };
        (
          argument4: Argument4,
          argument5: Argument5,
        ): (argument6: Argument6) => Result;
        (
          argument4: Argument4,
          argument5: Argument5,
          argument6: Argument6,
        ): Result;
      };
      (
        argument3: Argument3,
        argument4: Argument4,
      ): {
        (argument5: Argument5): (argument6: Argument6) => Result;
        (argument5: Argument5, argument6: Argument6): Result;
      };
      (
        argument3: Argument3,
        argument4: Argument4,
        argument5: Argument5,
      ): (argument6: Argument6) => Result;
      (
        argument3: Argument3,
        argument4: Argument4,
        argument5: Argument5,
        argument6: Argument6,
      ): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
    ): {
      (
        argument4: Argument4,
      ): {
        (argument5: Argument5): (argument6: Argument6) => Result;
        (argument5: Argument5, argument6: Argument6): Result;
      };
      (
        argument4: Argument4,
        argument5: Argument5,
      ): (argument6: Argument6) => Result;
      (
        argument4: Argument4,
        argument5: Argument5,
        argument6: Argument6,
      ): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
      argument4: Argument4,
    ): {
      (argument5: Argument5): (argument6: Argument6) => Result;
      (argument5: Argument5, argument6: Argument6): Result;
    };
    (
      argument2: Argument2,
      argument3: Argument3,
      argument4: Argument4,
      argument5: Argument5,
    ): (argument6: Argument6) => Result;
    (
      argument2: Argument2,
      argument3: Argument3,
      argument4: Argument4,
      argument5: Argument5,
      argument6: Argument6,
    ): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
  ): {
    (
      argument3: Argument3,
    ): {
      (
        argument4: Argument4,
      ): {
        (argument5: Argument5): (argument6: Argument6) => Result;
        (argument5: Argument5, argument6: Argument6): Result;
      };
      (
        argument4: Argument4,
        argument5: Argument5,
      ): (argument6: Argument6) => Result;
      (
        argument4: Argument4,
        argument5: Argument5,
        argument6: Argument6,
      ): Result;
    };
    (
      argument3: Argument3,
      argument4: Argument4,
    ): {
      (argument5: Argument5): (argument6: Argument6) => Result;
      (argument5: Argument5, argument6: Argument6): Result;
    };
    (
      argument3: Argument3,
      argument4: Argument4,
      argument5: Argument5,
    ): (argument6: Argument6) => Result;
    (
      argument3: Argument3,
      argument4: Argument4,
      argument5: Argument5,
      argument6: Argument6,
    ): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
  ): {
    (
      argument4: Argument4,
    ): {
      (argument5: Argument5): (argument6: Argument6) => Result;
      (argument5: Argument5, argument6: Argument6): Result;
    };
    (
      argument4: Argument4,
      argument5: Argument5,
    ): (argument6: Argument6) => Result;
    (argument4: Argument4, argument5: Argument5, argument6: Argument6): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
  ): {
    (argument5: Argument5): (argument6: Argument6) => Result;
    (argument5: Argument5, argument6: Argument6): Result;
  };
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
    argument5: Argument5,
  ): (argument6: Argument6) => Result;
  (
    argument1: Argument1,
    argument2: Argument2,
    argument3: Argument3,
    argument4: Argument4,
    argument5: Argument5,
    argument6: Argument6,
  ): Result;
};
export function curry(function_: (...arguments_: unknown[]) => unknown) {
  return function curried(this: unknown, ...arguments_: unknown[]) {
    if (arguments_.length >= function_.length) {
      return function_.apply(this, arguments_);
    }
    return function (this: unknown, ...moreArguments: unknown[]) {
      return Reflect.apply(curried, this, [...arguments_, ...moreArguments]);
    };
  };
}

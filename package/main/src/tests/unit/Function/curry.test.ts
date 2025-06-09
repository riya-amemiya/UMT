import { curry } from "@/Function/curry";

describe("curry", () => {
  test("should curry a function with 0 arguments", () => {
    const func = () => "hello";
    const curriedFunc = curry(func);
    expect(curriedFunc()).toBe("hello");
  });

  test("should curry a function with 1 argument", () => {
    const func = (a: number) => a * 2;
    const curriedFunc = curry(func);
    expect(curriedFunc(5)).toBe(10);
  });

  test("should curry a function with 2 arguments", () => {
    const func = (a: number, b: number) => a + b;
    const curriedFunc = curry(func);
    expect(curriedFunc(2)(3)).toBe(5);
    expect(curriedFunc(2, 3)).toBe(5);
  });

  test("should curry a function with 3 arguments", () => {
    const func = (a: number, b: number, c: number) => a * b + c;
    const curriedFunc = curry(func);
    expect(curriedFunc(2)(3)(4)).toBe(10);
    expect(curriedFunc(2, 3)(4)).toBe(10);
    expect(curriedFunc(2)(3, 4)).toBe(10);
    expect(curriedFunc(2, 3, 4)).toBe(10);
  });

  test("should curry a function with 4 arguments", () => {
    const func = (a: number, b: number, c: number, d: number) => a + b + c + d;
    const curriedFunc = curry(func);
    expect(curriedFunc(1)(2)(3)(4)).toBe(10);
    expect(curriedFunc(1, 2)(3)(4)).toBe(10);
    expect(curriedFunc(1)(2, 3)(4)).toBe(10);
    expect(curriedFunc(1)(2)(3, 4)).toBe(10);
    expect(curriedFunc(1, 2, 3)(4)).toBe(10);
    expect(curriedFunc(1, 2)(3, 4)).toBe(10);
    expect(curriedFunc(1, 2, 3, 4)).toBe(10);
  });

  test("should curry a function with 5 arguments", () => {
    const func = (a: number, b: number, c: number, d: number, e: number) =>
      a + b + c + d + e;
    const curriedFunc = curry(func);
    expect(curriedFunc(1)(2)(3)(4)(5)).toBe(15);
    expect(curriedFunc(1, 2)(3)(4)(5)).toBe(15);
    expect(curriedFunc(1)(2, 3)(4)(5)).toBe(15);
    expect(curriedFunc(1)(2)(3, 4)(5)).toBe(15);
    expect(curriedFunc(1)(2)(3)(4, 5)).toBe(15);
    expect(curriedFunc(1, 2, 3)(4)(5)).toBe(15);
    expect(curriedFunc(1, 2)(3, 4)(5)).toBe(15);
    expect(curriedFunc(1, 2)(3)(4, 5)).toBe(15);
    expect(curriedFunc(1, 2, 3, 4)(5)).toBe(15);
    expect(curriedFunc(1, 2, 3)(4, 5)).toBe(15);
    expect(curriedFunc(1, 2, 3, 4, 5)).toBe(15);
  });

  test("should curry a function with 6 arguments", () => {
    const func = (
      a: number,
      b: number,
      c: number,
      d: number,
      e: number,
      f: number,
    ) => a + b + c + d + e + f;
    const curriedFunc = curry(func);
    expect(curriedFunc(1)(2)(3)(4)(5)(6)).toBe(21);
    expect(curriedFunc(1, 2)(3)(4)(5)(6)).toBe(21);
    expect(curriedFunc(1)(2, 3)(4)(5)(6)).toBe(21);
    expect(curriedFunc(1)(2)(3, 4)(5)(6)).toBe(21);
    expect(curriedFunc(1)(2)(3)(4, 5)(6)).toBe(21);
    expect(curriedFunc(1)(2)(3)(4)(5, 6)).toBe(21);
    expect(curriedFunc(1, 2, 3)(4)(5)(6)).toBe(21);
    expect(curriedFunc(1, 2)(3, 4)(5)(6)).toBe(21);
    expect(curriedFunc(1, 2)(3)(4, 5)(6)).toBe(21);
    expect(curriedFunc(1, 2)(3)(4)(5, 6)).toBe(21);
    expect(curriedFunc(1, 2, 3, 4)(5)(6)).toBe(21);
    expect(curriedFunc(1, 2, 3)(4, 5)(6)).toBe(21);
    expect(curriedFunc(1, 2, 3)(4)(5, 6)).toBe(21);
    expect(curriedFunc(1, 2, 3, 4, 5)(6)).toBe(21);
    expect(curriedFunc(1, 2, 3, 4)(5, 6)).toBe(21);
    expect(curriedFunc(1, 2, 3, 4, 5, 6)).toBe(21);
  });

  test("should handle different argument types", () => {
    const func = (a: number, b: string, c: boolean) => `${a}${b}${c}`;
    const curriedFunc = curry(func);
    expect(curriedFunc(1)("hello")(true)).toBe("1hellotrue");
    expect(curriedFunc(1, "hello")(true)).toBe("1hellotrue");
    expect(curriedFunc(1)("hello", true)).toBe("1hellotrue");
    expect(curriedFunc(1, "hello", true)).toBe("1hellotrue");
  });

  // 追加テスト

  test("should curry a function with object and array arguments", () => {
    const func = (obj: { x: number }, arr: number[]) =>
      obj.x + arr.reduce((a, b) => a + b, 0);
    const curriedFunc = curry(func);
    expect(curriedFunc({ x: 5 })([1, 2, 3])).toBe(11);
    expect(curriedFunc({ x: 5 }, [1, 2, 3])).toBe(11);
  });

  test("should curry a function with function arguments", () => {
    const func = (f: (x: number) => number, x: number) => f(x);
    const curriedFunc = curry(func);
    const double = (x: number) => x * 2;
    expect(curriedFunc(double)(5)).toBe(10);
    expect(curriedFunc(double, 5)).toBe(10);
  });

  test("should preserve the original 'this' context", () => {
    const obj = {
      multiplier: 2,
      multiply: function (a: number, b: number) {
        return a * b * this.multiplier;
      },
    };

    const curriedMultiply = curry(obj.multiply.bind(obj));
    expect(curriedMultiply(3)(4)).toBe(24);
    expect(curriedMultiply(3, 4)).toBe(24);
  });
});

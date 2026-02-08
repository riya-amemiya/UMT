import { once } from "@/Function/once";

describe("once", () => {
  it("should invoke the function only once", () => {
    const fn = jest.fn(() => 42);
    const onceFn = once(fn);

    expect(onceFn()).toBe(42);
    expect(onceFn()).toBe(42);
    expect(onceFn()).toBe(42);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should pass arguments to the first call", () => {
    const fn = jest.fn((a: number, b: number) => a + b);
    const onceFn = once(fn);

    expect(onceFn(3, 4)).toBe(7);
    expect(onceFn(10, 20)).toBe(7);
    expect(fn).toHaveBeenCalledWith(3, 4);
  });

  it("should return the first result even when called with different arguments", () => {
    const fn = jest.fn((x: string) => x.toUpperCase());
    const onceFn = once(fn);

    expect(onceFn("hello")).toBe("HELLO");
    expect(onceFn("world")).toBe("HELLO");
  });

  it("should handle void functions", () => {
    let counter = 0;
    const fn = once(() => {
      counter++;
    });

    fn();
    fn();
    fn();
    expect(counter).toBe(1);
  });
});

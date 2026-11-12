import { update } from "@/Object/update";

describe("update", () => {
  it("updates a top-level property from its current value", () => {
    const target = { a: 1 };
    update(target, "a", (value) => (value as number) + 1);
    expect(target).toEqual({ a: 2 });
  });

  it("updates a nested property", () => {
    const target = { a: { b: 1 } };
    update(target, "a.b", (value) => (value as number) * 10);
    expect(target).toEqual({ a: { b: 10 } });
  });

  it("accepts an array path", () => {
    const target = { a: { b: "x" } };
    update(target, ["a", "b"], (value) => `${value as string}y`);
    expect(target).toEqual({ a: { b: "xy" } });
  });

  it("passes undefined and creates intermediates when the path is missing", () => {
    const target: Record<string, unknown> = {};
    const updater = jest.fn((value: unknown) => value ?? 0);
    update(target, "a.b", updater);
    expect(updater).toHaveBeenCalledWith(undefined);
    expect(target).toEqual({ a: { b: 0 } });
  });

  it("returns the same object reference", () => {
    const target = { a: 1 };
    expect(update(target, "a", () => 2)).toBe(target);
  });

  it("does not call the updater for an empty path array", () => {
    const target = { a: 1 };
    const updater = jest.fn();
    expect(update(target, [], updater)).toBe(target);
    expect(updater).not.toHaveBeenCalled();
    expect(target).toEqual({ a: 1 });
  });

  it("replaces non-object intermediate values", () => {
    const target: Record<string, unknown> = { a: 5 };
    update(target, "a.b", () => 1);
    expect(target).toEqual({ a: { b: 1 } });
  });
});

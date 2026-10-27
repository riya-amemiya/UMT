import type { ShallowObjectValue } from "$/object/shallowObjectValue";

describe("ShallowObjectValue", () => {
  it("ShallowObjectValue<{ a: { b: 1 }; c: { d: 2 } }>", () => {
    const _: ShallowObjectValue<{ a: { b: 1 }; c: { d: 2 } }> = { b: 1, d: 2 };
    expect(_).toEqual(_);
  });
  it("ShallowObjectValue<string>", () => {
    const _: [ShallowObjectValue<string>] extends [never] ? true : false = true;
    expect(_).toEqual(_);
  });
});

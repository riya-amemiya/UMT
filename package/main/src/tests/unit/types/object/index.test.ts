import type { PickPartial } from "$/object/index";

describe("object index", () => {
  it('PickPartial<{ a: 1; b: 2 }, "a"> without a', () => {
    const _: PickPartial<{ a: 1; b: 2 }, "a"> = { b: 2 };
    expect(_).toEqual(_);
  });
  it('PickPartial<{ a: 1; b: 2 }, "a"> with a', () => {
    const _: PickPartial<{ a: 1; b: 2 }, "a"> = { a: 1, b: 2 };
    expect(_).toEqual(_);
  });
});

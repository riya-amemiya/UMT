import type { GetEnumValues } from "$/enum/index";

describe("enum index", () => {
  it("GetEnumValues<{ a: 1; b: 2 }>", () => {
    const _: GetEnumValues<{ a: 1; b: 2 }>[] = [1, 2];
    expect(_).toEqual(_);
  });
  it("3 extends GetEnumValues<{ a: 1; b: 2 }>", () => {
    const _: 3 extends GetEnumValues<{ a: 1; b: 2 }> ? true : false = false;
    expect(_).toEqual(_);
  });
});

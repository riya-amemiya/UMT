import { unflattenObject } from "@/Object/unflattenObject";

describe("unflattenObject", () => {
  it("rebuilds a nested object from dotted paths", () => {
    expect(unflattenObject({ "a.b.c": 1 })).toEqual({ a: { b: { c: 1 } } });
  });

  it("merges multiple paths into shared parents", () => {
    expect(unflattenObject({ "a.b": 1, "a.c": 2 })).toEqual({
      a: { b: 1, c: 2 },
    });
  });

  it("supports custom separator", () => {
    expect(unflattenObject({ "a/b": 1 }, "/")).toEqual({ a: { b: 1 } });
  });

  it("preserves top-level scalars", () => {
    expect(unflattenObject({ a: 1, b: 2 })).toEqual({ a: 1, b: 2 });
  });

  it("returns empty for empty input", () => {
    expect(unflattenObject({})).toEqual({});
  });
});

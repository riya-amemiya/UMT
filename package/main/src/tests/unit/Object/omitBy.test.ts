import { omitBy } from "@/Object/omitBy";

describe("omitBy", () => {
  it("removes entries where predicate returns true", () => {
    expect(
      omitBy({ a: 1, b: undefined, c: 3 }, (v) => v === undefined),
    ).toEqual({ a: 1, c: 3 });
  });

  it("returns all entries when predicate always false", () => {
    expect(omitBy({ a: 1, b: 2 }, () => false)).toEqual({ a: 1, b: 2 });
  });

  it("returns empty when predicate always true", () => {
    expect(omitBy({ a: 1, b: 2 }, () => true)).toEqual({});
  });

  it("passes the key as second argument", () => {
    expect(omitBy({ a: 1, b: 2 }, (_, key) => key === "a")).toEqual({ b: 2 });
  });

  it("returns empty for empty input", () => {
    expect(omitBy({}, () => false)).toEqual({});
  });
});

import { pickBy } from "@/Object/pickBy";

describe("pickBy", () => {
  it("selects entries where predicate returns true", () => {
    expect(pickBy({ a: 1, b: 2, c: 3 }, (v) => v > 1)).toEqual({ b: 2, c: 3 });
  });

  it("returns empty when nothing matches", () => {
    expect(pickBy({ a: 1 }, () => false)).toEqual({});
  });

  it("returns all entries when everything matches", () => {
    expect(pickBy({ a: 1, b: 2 }, () => true)).toEqual({ a: 1, b: 2 });
  });

  it("passes the key as second argument", () => {
    expect(pickBy({ a: 1, b: 2 }, (_, key) => key === "a")).toEqual({ a: 1 });
  });

  it("returns empty for empty input", () => {
    expect(pickBy({}, () => true)).toEqual({});
  });

  it("does not mutate the input", () => {
    const input = { a: 1, b: 2 };
    pickBy(input, () => true);
    expect(input).toEqual({ a: 1, b: 2 });
  });
});

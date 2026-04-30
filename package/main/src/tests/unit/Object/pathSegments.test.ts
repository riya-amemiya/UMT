import { pathSegments } from "@/Object/pathSegments";

describe("pathSegments", () => {
  it("splits a dotted string into segments", () => {
    expect(pathSegments("a.b.c")).toEqual(["a", "b", "c"]);
  });

  it("returns the input array unchanged", () => {
    const input = ["a", "b"];
    expect(pathSegments(input)).toBe(input);
  });

  it("returns a single-element array for a non-dotted string", () => {
    expect(pathSegments("a")).toEqual(["a"]);
  });

  it("splits empty string into a single empty segment", () => {
    expect(pathSegments("")).toEqual([""]);
  });
});

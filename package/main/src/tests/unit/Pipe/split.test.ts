import { split } from "@/Pipe/split";
import { pipe } from "@/Tool/pipe";

describe("split", () => {
  it("splits the string by a string separator", () => {
    expect(pipe("a,b,c", split(","))).toEqual(["a", "b", "c"]);
  });

  it("splits the string by a regular expression", () => {
    expect(pipe("a1b22c", split(/\d+/))).toEqual(["a", "b", "c"]);
  });

  it("returns at most limit substrings", () => {
    expect(pipe("a,b,c", split(",", 2))).toEqual(["a", "b"]);
  });

  it("returns an array with an empty string for an empty string", () => {
    expect(pipe("", split(","))).toEqual([""]);
  });
});

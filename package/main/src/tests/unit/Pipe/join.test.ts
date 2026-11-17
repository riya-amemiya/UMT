import { join } from "@/Pipe/join";
import { pipe } from "@/Tool/pipe";

describe("join", () => {
  it("joins the elements with the separator", () => {
    expect(pipe(["a", "b", "c"], join("-"))).toBe("a-b-c");
  });

  it("joins the elements with a comma when the separator is omitted", () => {
    expect(pipe(["a", "b", "c"], join())).toBe("a,b,c");
  });

  it("returns an empty string for an empty array", () => {
    expect(pipe([], join("-"))).toBe("");
  });

  it("converts non-string elements to strings", () => {
    expect(pipe([1, true, null], join(" "))).toBe("1 true ");
  });
});

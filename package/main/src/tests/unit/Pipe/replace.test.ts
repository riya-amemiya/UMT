import { replace } from "@/Pipe/replace";
import { pipe } from "@/Tool/pipe";

describe("replace", () => {
  it("replaces the first match of a string pattern", () => {
    expect(pipe("a-b-c", replace("-", "+"))).toBe("a+b-c");
  });

  it("replaces every match of a global regular expression", () => {
    expect(pipe("a-b-c", replace(/-/g, "+"))).toBe("a+b+c");
  });

  it("returns the string unchanged when nothing matches", () => {
    expect(pipe("abc", replace("-", "+"))).toBe("abc");
  });
});

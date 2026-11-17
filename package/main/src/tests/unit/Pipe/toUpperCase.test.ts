import { toUpperCase } from "@/Pipe/toUpperCase";
import { pipe } from "@/Tool/pipe";

describe("toUpperCase", () => {
  it("converts the string to upper case", () => {
    expect(pipe("Hello world", toUpperCase())).toBe("HELLO WORLD");
  });

  it("keeps characters without case unchanged", () => {
    expect(pipe("123-_", toUpperCase())).toBe("123-_");
  });
});

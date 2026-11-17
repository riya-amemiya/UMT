import { trim } from "@/Pipe/trim";
import { pipe } from "@/Tool/pipe";

describe("trim", () => {
  it("removes leading and trailing whitespace", () => {
    expect(pipe("  hello  ", trim())).toBe("hello");
  });

  it("keeps whitespace inside the string", () => {
    expect(pipe(" a b ", trim())).toBe("a b");
  });

  it("removes tabs and newlines", () => {
    expect(pipe("\t\nhello\n", trim())).toBe("hello");
  });
});

import { toLowerCase } from "@/Pipe/toLowerCase";
import { pipe } from "@/Tool/pipe";

describe("toLowerCase", () => {
  it("converts the string to lower case", () => {
    expect(pipe("Hello WORLD", toLowerCase())).toBe("hello world");
  });

  it("keeps characters without case unchanged", () => {
    expect(pipe("123-_", toLowerCase())).toBe("123-_");
  });
});

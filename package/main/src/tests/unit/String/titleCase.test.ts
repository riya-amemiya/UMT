import { titleCase } from "@/String/titleCase";

describe("titleCase", () => {
  it("converts plain sentence", () => {
    expect(titleCase("hello world")).toBe("Hello World");
  });

  it("converts mixed separators", () => {
    expect(titleCase("a-quick brown_fox")).toBe("A Quick Brown Fox");
  });

  it("converts camelCase", () => {
    expect(titleCase("helloWorld")).toBe("Hello World");
  });

  it("returns empty for empty string", () => {
    expect(titleCase("")).toBe("");
  });
});

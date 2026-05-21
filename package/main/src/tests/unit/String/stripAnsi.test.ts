import { stripAnsi } from "@/String/stripAnsi";

describe("stripAnsi", () => {
  it("should remove SGR color codes", () => {
    expect(stripAnsi("\u001B[31mred\u001B[0m")).toBe("red");
  });

  it("should remove multiple sequences", () => {
    expect(stripAnsi("\u001B[1m\u001B[32mbold green\u001B[0m")).toBe(
      "bold green",
    );
  });

  it("should remove cursor movement sequences", () => {
    expect(stripAnsi("a\u001B[2Ab")).toBe("ab");
  });

  it("should leave plain text unchanged", () => {
    expect(stripAnsi("plain text")).toBe("plain text");
  });

  it("should handle empty string", () => {
    expect(stripAnsi("")).toBe("");
  });
});

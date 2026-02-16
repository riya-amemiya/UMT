import { truncate } from "@/String/truncate";

describe("truncate", () => {
  it("should truncate string according to JSDoc examples", () => {
    expect(truncate("Hello World", 5)).toBe("Hello...");
    expect(truncate("Hello World", 5, "~")).toBe("Hello~");
    expect(truncate("Hello", 10)).toBe("Hello");
  });

  it("should not truncate if string is shorter than or equal to length", () => {
    expect(truncate("Hi", 5)).toBe("Hi");
    expect(truncate("Hello", 5)).toBe("Hello");
  });

  it("should handle empty suffix", () => {
    expect(truncate("Hello World", 5, "")).toBe("Hello");
  });

  it("should handle zero length", () => {
    expect(truncate("Hello", 0, "")).toBe("");
    expect(truncate("Hello", 0)).toBe("...");
  });

  it("should handle suffix longer than target length", () => {
    expect(truncate("Hello World", 2, "...")).toBe("He...");
    expect(truncate("Hello World", 1, "...")).toBe("H...");
  });

  it("should handle empty string", () => {
    expect(truncate("", 5)).toBe("");
  });
});

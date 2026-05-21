import { swapCase } from "@/String/swapCase";

describe("swapCase", () => {
  it("should swap the case of each letter", () => {
    expect(swapCase("Hello World")).toBe("hELLO wORLD");
  });

  it("should turn all uppercase into lowercase", () => {
    expect(swapCase("ABC")).toBe("abc");
  });

  it("should turn all lowercase into uppercase", () => {
    expect(swapCase("abc")).toBe("ABC");
  });

  it("should leave characters without case unchanged", () => {
    expect(swapCase("abc123-日本")).toBe("ABC123-日本");
  });

  it("should handle empty string", () => {
    expect(swapCase("")).toBe("");
  });
});

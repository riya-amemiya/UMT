import { calculator } from "@/Math/calculator/calculator";

describe("calculator - security/regex tests", () => {
  it("should handle currency symbols with special regex characters correctly", () => {
    // Symbol "A+B"
    // Current insecure regex: /\A+B([0-9]+)/
    // Matches "AAB100", "AAAB100" etc.
    // Does NOT match "A+B100" because + is treated as quantifier.
    // includes("A+B") passes.
    // So replacement fails.

    const rates = { "A+B": 2 };
    // "A+B100" -> should become "200"
    const result = calculator("A+B100", rates);
    expect(result).toBe("200");
  });

  it("should handle currency symbols with other special characters", () => {
    const rates = { US$: 1 };
    const result = calculator("US$100", rates);
    expect(result).toBe("100");
  });

  it("should handle currency symbols with brackets", () => {
    const rates = { "[X]": 2 };
    const result = calculator("[X]100", rates);
    expect(result).toBe("200");
  });
});

import { mathSeparator } from "@/Math/mathSeparator";
describe("mathSeparator", () => {
  // Test number type inputs
  it("should handle single digit numbers", () => {
    expect(mathSeparator(5)).toEqual([5, 0]);
  });

  it("should handle multi-digit numbers", () => {
    expect(mathSeparator(1250)).toEqual([1000, 250]);
  });

  // Test string number inputs
  it("should handle single digit string numbers", () => {
    expect(mathSeparator("5")).toEqual([5, 0]);
  });

  it("should handle multi-digit string numbers", () => {
    expect(mathSeparator("1250")).toEqual([1000, 250]);
  });

  // Test zero cases
  it("should handle 0", () => {
    expect(mathSeparator(0)).toEqual([0, 0]);
  });

  it('should handle "0"', () => {
    expect(mathSeparator("0")).toEqual([0, 0]);
  });

  // Test invalid inputs
  it("should return [0, 0] for non-number input", () => {
    expect(mathSeparator("abc")).toEqual([0, 0]);
  });

  // Test decimal numbers
  it("should handle decimal numbers", () => {
    expect(mathSeparator(12.5)).toEqual([10, 2.5]);
  });

  it("should handle decimal string numbers", () => {
    expect(mathSeparator("12.5")).toEqual([10, 2.5]);
  });

  // Test edge cases
  it("should handle negative numbers", () => {
    expect(mathSeparator(-1250)).toEqual([10_000, -11_250]);
    expect(mathSeparator("-1250.5")).toEqual([10_000, -11_249.5]);
  });

  it("should handle very large numbers", () => {
    expect(mathSeparator(1_000_000)).toEqual([1_000_000, 0]);
    expect(mathSeparator("1000000.123")).toEqual([1_000_000, 0.123]);
  });

  it("should handle special numeric strings", () => {
    // Note: "1e5" is treated as a regular string "1e5"
    expect(mathSeparator("1e5")).toEqual([100, 99_900]);
    // Note: isNumber treats "0xFF" as a valid number, so it's processed accordingly
    expect(mathSeparator("0xFF")).toEqual([1000, -745]);
    // Note: "Infinity" is not a valid number in this context
    expect(mathSeparator("Infinity")).toEqual([0, 0]);
  });
});

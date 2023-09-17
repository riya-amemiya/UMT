import { addition } from "@/Math/addition";

describe("addition", () => {
  it("should add two positive integers", () => {
    expect(addition(2, 3)).toBe(5);
  });

  it("should add two negative integers", () => {
    expect(addition(-2, -3)).toBe(-5);
  });

  it("should add a positive and a negative integer", () => {
    expect(addition(2, -3)).toBe(-1);
  });

  it("should add two positive decimal numbers", () => {
    expect(addition(0.1, 0.2)).toBeCloseTo(0.3);
  });

  it("should add two negative decimal numbers", () => {
    expect(addition(-0.1, -0.2)).toBeCloseTo(-0.3);
  });

  it("should add a positive and a negative decimal number", () => {
    expect(addition(0.1, -0.2)).toBeCloseTo(-0.1);
  });

  it("should add a positive integer and a positive decimal number", () => {
    expect(addition(2, 0.3)).toBeCloseTo(2.3);
  });

  it("should add a negative integer and a negative decimal number", () => {
    expect(addition(-2, -0.3)).toBeCloseTo(-2.3);
  });

  it("should add a positive integer and a negative decimal number", () => {
    expect(addition(2, -0.3)).toBeCloseTo(1.7);
  });

  it("should add a negative integer and a positive decimal number", () => {
    expect(addition(-2, 0.3)).toBeCloseTo(-1.7);
  });
});

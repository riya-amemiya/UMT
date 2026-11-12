import { mapRange } from "@/Math/mapRange";

describe("mapRange function", () => {
  it("should map a value between ranges", () => {
    expect(mapRange(5, 0, 10, 0, 100)).toBe(50);
    expect(mapRange(0.5, 0, 1, 10, 20)).toBe(15);
    expect(mapRange(25, 0, 100, -1, 1)).toBe(-0.5);
  });

  it("should map range endpoints to output endpoints", () => {
    expect(mapRange(0, 0, 10, 100, 200)).toBe(100);
    expect(mapRange(10, 0, 10, 100, 200)).toBe(200);
  });

  it("should map onto an inverted output range", () => {
    expect(mapRange(2.5, 0, 10, 100, 0)).toBe(75);
  });

  it("should not clamp values outside the input range", () => {
    expect(mapRange(15, 0, 10, 0, 100)).toBe(150);
    expect(mapRange(-5, 0, 10, 0, 100)).toBe(-50);
  });

  it("should return outMin when the input range is empty", () => {
    expect(mapRange(7, 5, 5, 10, 20)).toBe(10);
  });
});

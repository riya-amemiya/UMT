import { hexToRgba } from "@/Tool/hexToRgba";

describe("hexToRgba", () => {
  it("should convert valid hex code to rgba", () => {
    expect(hexToRgba("#FF0000")).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(hexToRgba("#00FF00")).toEqual({ r: 0, g: 255, b: 0, a: 1 });
    expect(hexToRgba("#0000FF")).toEqual({ r: 0, g: 0, b: 255, a: 1 });
    expect(hexToRgba("#FFFFFF")).toEqual({ r: 255, g: 255, b: 255, a: 1 });
    expect(hexToRgba("#000000")).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hexToRgba("#FFA500")).toEqual({ r: 255, g: 165, b: 0, a: 1 });
  });

  it("should throw an error for invalid hex code", () => {
    expect(() => hexToRgba("#12345")).toThrow("Invalid hex code");
    expect(() => hexToRgba("#1234567")).toThrow("Invalid hex code");
    expect(() => hexToRgba("123456")).toThrow("Invalid hex code");
    expect(() => hexToRgba("")).toThrow("Invalid hex code");
  });
});

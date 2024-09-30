import { hexaToRgba } from "@/Color/hexaToRgba";

describe("hexaToRgba", () => {
  it("should convert valid hex code to rgba", () => {
    expect(hexaToRgba("#FF0000")).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#00FF00")).toEqual({ r: 0, g: 255, b: 0, a: 1 });
    expect(hexaToRgba("#0000FF")).toEqual({ r: 0, g: 0, b: 255, a: 1 });
    expect(hexaToRgba("#FFFFFF")).toEqual({ r: 255, g: 255, b: 255, a: 1 });
    expect(hexaToRgba("#000000")).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#FFA500")).toEqual({ r: 255, g: 165, b: 0, a: 1 });
    expect(hexaToRgba("#FFA50099")).toEqual({ r: 255, g: 165, b: 0, a: 0.6 });
  });

  it("should throw an error for invalid hex code", () => {
    expect(() => hexaToRgba("#12345")).toThrow("Invalid hex code");
    expect(() => hexaToRgba("#1234567")).toThrow("Invalid hex code");
    expect(() => hexaToRgba("123456")).toThrow("Invalid hex code");
    expect(() => hexaToRgba("")).toThrow("Invalid hex code");
  });
});

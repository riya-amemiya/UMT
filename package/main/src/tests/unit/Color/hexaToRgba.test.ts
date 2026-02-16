import { hexaToRgba } from "@/Color/hexaToRgba";

describe("hexaToRgba", () => {
  it("should convert 6-digit hex code to rgba", () => {
    expect(hexaToRgba("#FF0000")).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#00FF00")).toEqual({ r: 0, g: 255, b: 0, a: 1 });
    expect(hexaToRgba("#0000FF")).toEqual({ r: 0, g: 0, b: 255, a: 1 });
    expect(hexaToRgba("#FFFFFF")).toEqual({ r: 255, g: 255, b: 255, a: 1 });
    expect(hexaToRgba("#000000")).toEqual({ r: 0, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#FFA500")).toEqual({ r: 255, g: 165, b: 0, a: 1 });
    expect(hexaToRgba("#FFA50099")).toEqual({ r: 255, g: 165, b: 0, a: 0.6 });
  });

  it("should convert 3-digit hex code to rgba", () => {
    expect(hexaToRgba("#F00")).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#0F0")).toEqual({ r: 0, g: 255, b: 0, a: 1 });
    expect(hexaToRgba("#00F")).toEqual({ r: 0, g: 0, b: 255, a: 1 });
    expect(hexaToRgba("#FFF")).toEqual({ r: 255, g: 255, b: 255, a: 1 });
    expect(hexaToRgba("#000")).toEqual({ r: 0, g: 0, b: 0, a: 1 });
  });

  it("should convert 8-digit hex code to rgba", () => {
    expect(hexaToRgba("#FF0000FF")).toEqual({ r: 255, g: 0, b: 0, a: 1 });
    expect(hexaToRgba("#FF000080")).toEqual({ r: 255, g: 0, b: 0, a: 0.5 });
    expect(hexaToRgba("#FF000000")).toEqual({ r: 255, g: 0, b: 0, a: 0 });
  });
});

import { rgbaToHexA } from "@/Color/rgbaToHexA";

describe("rgbaToHexA", () => {
  it("should convert valid rgba to hex code", () => {
    expect(rgbaToHexA({ r: 255, g: 0, b: 0 })).toEqual("#ff0000ff");
    expect(rgbaToHexA({ r: 0, g: 255, b: 0, a: 1 })).toEqual("#00ff00ff");
    expect(rgbaToHexA({ r: 0, g: 0, b: 255, a: 1 })).toEqual("#0000ffff");
    expect(rgbaToHexA({ r: 255, g: 255, b: 255, a: 1 })).toEqual("#ffffffff");
    expect(rgbaToHexA({ r: 0, g: 0, b: 0, a: 1 })).toEqual("#000000ff");
    expect(rgbaToHexA({ r: 255, g: 165, b: 0, a: 1 })).toEqual("#ffa500ff");
    expect(rgbaToHexA({ r: 255, g: 255, b: 255, a: 0.5 })).toEqual("#ffffff80");
  });

  it("should throw an error for invalid rgba", () => {
    expect(() => rgbaToHexA({ r: 256, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: 256, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: 0, b: 256, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: 0, b: 0, a: 1.5 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: -1, g: 0, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: -1, b: 0, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: 0, b: -1, a: 1 })).toThrow(
      "Invalid rgba value",
    );
    expect(() => rgbaToHexA({ r: 0, g: 0, b: 0, a: -0.5 })).toThrow(
      "Invalid rgba value",
    );
  });
});

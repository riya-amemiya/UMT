import type { TripleDigit } from "$/int/tripleDigit";

describe("TripleDigit", () => {
  it("TripleDigit with every leading digit", () => {
    const _: TripleDigit[] = [
      "100",
      "200",
      "300",
      "400",
      "500",
      "600",
      "700",
      "800",
      "900",
    ];
    expect(_).toEqual(_);
  });
  it("TripleDigit with every trailing digit", () => {
    const _: TripleDigit[] = [
      "100",
      "111",
      "122",
      "133",
      "144",
      "155",
      "166",
      "177",
      "188",
      "199",
    ];
    expect(_).toEqual(_);
  });
  it('"099" extends TripleDigit', () => {
    const _: "099" extends TripleDigit ? true : false = false;
    expect(_).toEqual(_);
  });
  it('"1000" extends TripleDigit', () => {
    const _: "1000" extends TripleDigit ? true : false = false;
    expect(_).toEqual(_);
  });
});

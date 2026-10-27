import type { PickPartial } from "$/object/pickPartial";

interface Sample {
  a: { b: string };
  c: number;
}

describe("PickPartial", () => {
  it('PickPartial<Sample, "c"> without c', () => {
    const _: PickPartial<Sample, "c"> = { a: { b: "x" } };
    expect(_).toEqual(_);
  });
  it('PickPartial<Sample, "c"> with c', () => {
    const _: PickPartial<Sample, "c"> = { a: { b: "x" }, c: 1 };
    expect(_).toEqual(_);
  });
  it('{ c: number } extends PickPartial<Sample, "c">', () => {
    const _: { c: number } extends PickPartial<Sample, "c"> ? true : false =
      false;
    expect(_).toEqual(_);
  });
});

import type { OmitDeep } from "$/object/omitDeep";

interface Sample {
  a: { b: string; c: number };
  d: boolean;
  e: number;
}

describe("OmitDeep", () => {
  it('OmitDeep<Sample, ["a.b"]>', () => {
    const _: OmitDeep<Sample, ["a.b"]> = { a: { c: 1 }, d: true, e: 1 };
    expect(_).toEqual(_);
  });
  it('OmitDeep<Sample, ["a.b"]> rejects the removed key', () => {
    const _: OmitDeep<Sample, ["a.b"]> = {
      // @ts-expect-error
      a: { b: "x", c: 1 },
      d: true,
      e: 1,
    };
    expect(_).toEqual(_);
  });
  it('OmitDeep<Sample, ["d", "e"]>', () => {
    const _: OmitDeep<Sample, ["d", "e"]> = { a: { b: "x", c: 1 } };
    expect(_).toEqual(_);
  });
  it('OmitDeep<Sample, ["a.b", "a.c", "d"]>', () => {
    const _: OmitDeep<Sample, ["a.b", "a.c", "d"]> = { a: {}, e: 1 };
    expect(_).toEqual(_);
  });
  it('OmitDeep<Sample, ["x.y"]>', () => {
    const _: OmitDeep<Sample, ["x.y"]> = { a: { b: "x", c: 1 }, d: true, e: 1 };
    expect(_).toEqual(_);
  });
  it('OmitDeep<Sample, ["d.x"]>', () => {
    const _: OmitDeep<Sample, ["d.x"]> = { a: { b: "x", c: 1 }, d: true, e: 1 };
    expect(_).toEqual(_);
  });
  it("OmitDeep<{ 1: string; b: number }, [1]>", () => {
    const _: OmitDeep<{ 1: string; b: number }, [1]> = { b: 1 };
    expect(_).toEqual(_);
  });
  it("OmitDeep<Sample, []>", () => {
    const _: OmitDeep<Sample, []> = { a: { b: "x", c: 1 }, d: true, e: 1 };
    expect(_).toEqual(_);
  });
});

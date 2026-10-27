import type { PickDeepKey } from "$/object/pickDeepKey";

interface Sample {
  a: { b: string };
  c: number;
}

describe("PickDeepKey", () => {
  it("PickDeepKey<Sample>", () => {
    const _: PickDeepKey<Sample>[] = ["a", "a.b", "c"];
    expect(_).toEqual(_);
  });
  it('"a.c" extends PickDeepKey<Sample>', () => {
    const _: "a.c" extends PickDeepKey<Sample> ? true : false = false;
    expect(_).toEqual(_);
  });
  it('PickDeepKey<{ a: { 1: "x" } }>', () => {
    const _: PickDeepKey<{ a: { 1: "x" } }> = "a";
    expect(_).toEqual(_);
  });
  it("PickDeepKey<{ 1: { b: 1 } }>", () => {
    const _: [PickDeepKey<{ 1: { b: 1 } }>] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
});

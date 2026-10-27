import type {
  ConstructNestedObject,
  GetValueAtPath,
  PickDeep,
  ProcessKeys,
} from "$/object/pickDeep";

interface Sample {
  a: { b: string };
  c: number;
}

describe("PickDeep", () => {
  it('GetValueAtPath<Sample, "a.b">', () => {
    const _: GetValueAtPath<Sample, "a.b"> = "x";
    expect(_).toEqual(_);
  });
  it('GetValueAtPath<Sample, "c.d">', () => {
    const _: [GetValueAtPath<Sample, "c.d">] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('GetValueAtPath<Sample, "x.y">', () => {
    const _: [GetValueAtPath<Sample, "x.y">] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('GetValueAtPath<Sample, "a">', () => {
    const _: GetValueAtPath<Sample, "a"> = { b: "x" };
    expect(_).toEqual(_);
  });
  it('GetValueAtPath<Sample, "x">', () => {
    const _: [GetValueAtPath<Sample, "x">] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('ConstructNestedObject<"a.b", number>', () => {
    const _: ConstructNestedObject<"a.b", number> = { a: { b: 1 } };
    expect(_).toEqual(_);
  });
  it('ConstructNestedObject<"a", number>', () => {
    const _: ConstructNestedObject<"a", number> = { a: 1 };
    expect(_).toEqual(_);
  });
  it('ProcessKeys<Sample, ["a.b", "c"]>', () => {
    const _: ProcessKeys<Sample, ["a.b", "c"]>[] = [
      { a: { b: "x" } },
      { c: 1 },
    ];
    expect(_).toEqual(_);
  });
  it('ProcessKeys<{ 1: "x" }, [1]>', () => {
    const _: [ProcessKeys<{ 1: "x" }, [1]>] extends [never] ? true : false =
      true;
    expect(_).toEqual(_);
  });
  it('PickDeep<Sample, ["a.b", "c"]>', () => {
    const _: PickDeep<Sample, ["a.b", "c"]> = { a: { b: "x" }, c: 1 };
    expect(_).toEqual(_);
  });
});

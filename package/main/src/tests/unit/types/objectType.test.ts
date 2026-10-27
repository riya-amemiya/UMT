import type {
  ConstructNestedObject,
  GetValueAtPath,
  PickDeep,
  ProcessKeys,
} from "$/object/pickDeep";
import type { PickDeepKey } from "$/object/pickDeepKey";
import type { PickPartial } from "$/object/pickPartial";
import type { ShallowObjectValue } from "$/object/shallowObjectValue";

interface Sample {
  a: { b: string };
  c: number;
}

describe("objectType", () => {
  it("pickDeepKey", () => {
    const _: [
      PickDeepKey<Sample>[],
      "a.c" extends PickDeepKey<Sample> ? true : false,
    ] = [["a", "a.b", "c"], false];
    expect(_).toEqual(_);
  });
  it("getValueAtPath", () => {
    const _: [
      GetValueAtPath<Sample, "a.b">,
      GetValueAtPath<Sample, "a">,
      [GetValueAtPath<Sample, "x">] extends [never] ? true : false,
      [GetValueAtPath<Sample, "x.y">] extends [never] ? true : false,
      [GetValueAtPath<Sample, "c.d">] extends [never] ? true : false,
    ] = ["x", { b: "x" }, true, true, true];
    expect(_).toEqual(_);
  });
  it("constructNestedObject", () => {
    const _: [
      ConstructNestedObject<"a.b", number>,
      ConstructNestedObject<"a", number>,
    ] = [{ a: { b: 1 } }, { a: 1 }];
    expect(_).toEqual(_);
  });
  it("processKeys", () => {
    const _: ProcessKeys<Sample, ["a.b", "c"]>[] = [
      { a: { b: "x" } },
      { c: 1 },
    ];
    expect(_).toEqual(_);
  });
  it("pickDeep", () => {
    const _: PickDeep<Sample, ["a.b", "c"]> = { a: { b: "x" }, c: 1 };
    expect(_).toEqual(_);
  });
  it("pickPartial", () => {
    const _: [
      PickPartial<Sample, "c">,
      undefined extends PickPartial<Sample, "c">["c"] ? true : false,
    ] = [{ a: { b: "x" } }, true];
    expect(_).toEqual(_);
  });
  it("shallowObjectValue", () => {
    const _: [
      ShallowObjectValue<{ a: { b: 1 }; c: { d: 2 } }>,
      [ShallowObjectValue<string>] extends [never] ? true : false,
    ] = [{ b: 1, d: 2 }, true];
    expect(_).toEqual(_);
  });
});

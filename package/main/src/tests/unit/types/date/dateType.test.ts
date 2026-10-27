import type { DateType } from "$/date/dateType";

describe("DateType", () => {
  it('DateType<"2"> with hyphens', () => {
    const _: DateType<"2">[] = ["2024-2-1", "2024-02-29"];
    expect(_).toEqual(_);
  });
  it('DateType<"2"> with slashes', () => {
    const _: DateType<"2">[] = ["2024/2/1", "2024/02/29"];
    expect(_).toEqual(_);
  });
  it('DateType<"2"> with colons', () => {
    const _: DateType<"2">[] = ["2024:2:1", "2024:02:29"];
    expect(_).toEqual(_);
  });
  it('"2024-02-30" extends DateType<"2">', () => {
    const _: "2024-02-30" extends DateType<"2"> ? true : false = false;
    expect(_).toEqual(_);
  });
  it("DateType<2> with hyphens", () => {
    const _: DateType<2>[] = ["2024-2-1", "2024-02-29"];
    expect(_).toEqual(_);
  });
  it("DateType<2> with slashes", () => {
    const _: DateType<2>[] = ["2024/2/1", "2024/02/29"];
    expect(_).toEqual(_);
  });
  it("DateType<2> with colons", () => {
    const _: DateType<2>[] = ["2024:2:1", "2024:02:29"];
    expect(_).toEqual(_);
  });
  it('"2024-2-30" extends DateType<2>', () => {
    const _: "2024-2-30" extends DateType<2> ? true : false = false;
    expect(_).toEqual(_);
  });
  it('DateType<"12">', () => {
    const _: DateType<"12">[] = ["2024-12-31", "2024/12/01", "2024:12:1"];
    expect(_).toEqual(_);
  });
});

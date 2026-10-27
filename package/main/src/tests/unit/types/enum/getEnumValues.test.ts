import type { GetEnumValues } from "$/enum/getEnumValues";

describe("GetEnumValues", () => {
  it("GetEnumValues<typeof colors>", () => {
    const colors = { red: "RED", blue: "BLUE" } as const;
    const _: GetEnumValues<typeof colors>[] = ["RED", "BLUE"];
    expect(_).toEqual(_);
  });
  it('"GREEN" extends GetEnumValues<typeof colors>', () => {
    const colors = { red: "RED", blue: "BLUE" } as const;
    const _: "GREEN" extends GetEnumValues<typeof colors> ? true : false =
      false;
    expect(_).toEqual(_);
  });
});

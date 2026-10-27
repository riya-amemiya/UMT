import type { GetEnumValues } from "$/enum/getEnumValues";

describe("enumType", () => {
  it("getEnumValues", () => {
    const colors = { red: "RED", blue: "BLUE" } as const;
    const _: GetEnumValues<typeof colors>[] = ["RED", "BLUE"];
    expect(_).toEqual(_);
  });
});

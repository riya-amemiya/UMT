import { toUnix } from "@/Date/toUnix";

describe("toUnix", () => {
  it("returns floored seconds by default", () => {
    expect(toUnix(new Date(0))).toBe(0);
    expect(toUnix(new Date(1_700_000_000_999))).toBe(1_700_000_000);
  });

  it("returns milliseconds when unit is ms", () => {
    const date = new Date(1_700_000_000_123);
    expect(toUnix(date, "ms")).toBe(1_700_000_000_123);
  });

  it("returns floored seconds when unit is s", () => {
    expect(toUnix(new Date(1_500), "s")).toBe(1);
  });
});

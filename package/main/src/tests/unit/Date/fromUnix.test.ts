import { fromUnix } from "@/Date/fromUnix";

describe("fromUnix", () => {
  it("creates a Date from seconds by default", () => {
    expect(fromUnix(0).getTime()).toBe(0);
    expect(fromUnix(1_700_000_000).getTime()).toBe(1_700_000_000_000);
  });

  it("creates a Date from milliseconds when unit is ms", () => {
    expect(fromUnix(1_700_000_000_123, "ms").getTime()).toBe(1_700_000_000_123);
  });

  it("creates a Date from seconds when unit is s", () => {
    expect(fromUnix(1_700_000_000, "s").getTime()).toBe(1_700_000_000_000);
  });

  it("treats an unknown unit as seconds", () => {
    // @ts-expect-error
    expect(fromUnix(1_700_000_000, "unknown").getTime()).toBe(
      1_700_000_000_000,
    );
  });
});

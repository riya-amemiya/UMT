import { isInRange } from "@/IP/isInRange";

describe("isInRange", () => {
  it("returns true for an IP within the range", () => {
    expect(isInRange("192.168.1.2", "192.168.1.0", 24)).toBe(true);
  });

  it("returns false for an IP outside the range", () => {
    expect(isInRange("192.168.2.2", "192.168.1.0", 24)).toBe(false);
  });
});

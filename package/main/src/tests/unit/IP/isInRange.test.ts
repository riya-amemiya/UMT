import { isInRange } from "@/IP/isInRange";

describe("isInRange", () => {
  describe("valid IP ranges", () => {
    test.each([
      // [remoteIp, networkIp, cidr, expected]
      ["192.168.1.2", "192.168.1.0", 24, true], // Common class C network
      ["192.168.2.2", "192.168.1.0", 24, false], // Outside class C network
      ["10.0.0.5", "10.0.0.0", 8, true], // Class A network
      ["11.0.0.5", "10.0.0.0", 8, false], // Outside class A network
      ["172.16.1.1", "172.16.0.0", 16, true], // Class B network
      ["172.17.1.1", "172.16.0.0", 16, false], // Outside class B network
      ["192.168.1.0", "192.168.1.0", 32, true], // Single host network - exact match
      ["192.168.1.1", "192.168.1.1", 32, true], // Single host network - exact match
      ["192.168.1.1", "192.168.1.0", 32, false], // Single host network - different IP
      ["192.168.1.1", "192.168.1.0", 30, true], // Small subnet
      ["192.168.1.4", "192.168.1.0", 30, false], // Outside small subnet
      ["0.0.0.1", "0.0.0.0", 0, true], // All IPs in range (CIDR 0)
      ["255.255.255.255", "0.0.0.0", 0, true], // All IPs in range (CIDR 0)
    ])(
      "should evaluate if %s is in network %s/%i => %s",
      (remoteIp, networkIp, cidr, expected) => {
        expect(isInRange(remoteIp, networkIp, cidr)).toBe(expected);
      },
    );

    test.each([
      // [remoteIp, networkIp, cidr, description]
      ["192.168.1.1", "0.0.0.0", 0, "CIDR 0: all IPs in range"],
      ["192.168.1.0", "192.168.1.0", 32, "CIDR 32: exact match only"],
      ["192.168.1.1", "192.168.1.0", 31, "CIDR 31: two IPs"],
      ["192.168.1.1", "192.168.1.0", 1, "CIDR 1: half of all IPs"],
    ])("should handle %s (%s)", (ip, network, cidr) => {
      expect(isInRange(ip, network, cidr)).toBe(true);
    });

    test("should correctly handle edge cases with specific CIDR masks", () => {
      expect(isInRange("192.168.1.5", "192.168.1.0", 24)).toBe(true);
      expect(isInRange("192.168.2.0", "192.168.1.0", 24)).toBe(false);

      expect(isInRange("192.168.1.0", "192.168.1.0", 31)).toBe(true);
      expect(isInRange("192.168.1.1", "192.168.1.0", 31)).toBe(true);
      expect(isInRange("192.168.1.2", "192.168.1.0", 31)).toBe(false);

      expect(isInRange("192.168.1.3", "192.168.1.0", 30)).toBe(true);
      expect(isInRange("192.168.1.4", "192.168.1.4", 30)).toBe(true);
    });
  });

  describe("invalid inputs", () => {
    test.each([
      [undefined, "192.168.1.0", 24, "Remote IP address is required"],
      ["192.168.1.1", undefined, 24, "Network IP address is required"],
      ["", "192.168.1.0", 24, "Remote IP address is required"],
      ["192.168.1.1", "", 24, "Network IP address is required"],
      [
        "192.168.1.1",
        "192.168.1.0",
        undefined,
        "CIDR must be an integer between 0 and 32",
      ],
      [
        "192.168.1.1",
        "192.168.1.0",
        -1,
        "CIDR must be an integer between 0 and 32",
      ],
      [
        "192.168.1.1",
        "192.168.1.0",
        33,
        "CIDR must be an integer between 0 and 32",
      ],
      [
        "192.168.1.1",
        "192.168.1.0",
        1.5,
        "CIDR must be an integer between 0 and 32",
      ],
    ])(
      "should throw error for remoteIp=%s, networkIp=%s, cidr=%s",
      (remoteIp, networkIp, cidr, expectedError) => {
        expect(() =>
          isInRange(remoteIp as string, networkIp as string, cidr as number),
        ).toThrow(expectedError);
      },
    );
  });

  describe("invalid IP formats", () => {
    test.each([
      ["invalid", "192.168.1.0", 24],
      ["192.168.1.1", "invalid", 24],
      ["256.256.256.256", "192.168.1.0", 24],
      ["192.168.1.1", "256.256.256.256", 24],
      ["192.168.1.1.1", "192.168.1.0", 24],
      ["192.168.-1.1", "192.168.1.0", 24],
      ["192.168.1", "192.168.1.0", 24],
    ])(
      "should throw for invalid IP format: %s",
      (remoteIp, networkIp, cidr) => {
        expect(() => isInRange(remoteIp, networkIp, cidr)).toThrow(
          "Invalid IP address format",
        );
      },
    );
  });

  describe("error message formatting", () => {
    test("should format error messages correctly with String(error) for invalid IP", () => {
      expect(() => isInRange("invalid-ip", "192.168.1.0", 24)).toThrow(
        "Invalid IP address format: Error: Invalid IP address format",
      );
    });

    test("should format error messages correctly for invalid network IP", () => {
      expect(() => isInRange("192.168.1.1", "invalid-network", 24)).toThrow(
        "Invalid IP address format: Error: Invalid IP address format",
      );
    });

    test("should format error messages for out-of-range IP values", () => {
      expect(() => isInRange("999.999.999.999", "192.168.1.0", 24)).toThrow(
        "Invalid IP address format: Error: Invalid IP address format",
      );
    });
  });
});

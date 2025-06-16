import {
  ipToLong,
  longToIp,
  cidrToSubnetMask,
  subnetMaskToCidr,
  ipToBinaryString,
  getNetworkAddress,
  isInRange,
  isPrivateIp,
  getIpClass,
} from "@/IP";

/**
 * Integration tests for IP address utility functions
 *
 * Tests the interaction between IP conversion functions:
 * - Round-trip conversions (IP ↔ Long ↔ Binary)
 * - CIDR and subnet mask conversions
 * - Network calculations with validation
 */
describe("Integration test for IP address conversions", () => {
  it("should perform round-trip conversion: IP → Long → IP", () => {
    const testIps = [
      "192.168.1.1",
      "10.0.0.1",
      "172.16.0.1",
      "8.8.8.8",
      "255.255.255.255",
      "0.0.0.0",
    ];

    testIps.forEach((originalIp) => {
      const longValue = ipToLong(originalIp);
      const convertedIp = longToIp(longValue);

      expect(convertedIp).toBe(originalIp);
    });
  });

  it("should perform round-trip conversion: CIDR → Subnet Mask → CIDR", () => {
    const cidrValues = [8, 16, 24, 30, 32];

    cidrValues.forEach((originalCidr) => {
      const subnetMask = cidrToSubnetMask(originalCidr);
      const convertedCidr = subnetMaskToCidr(subnetMask);

      expect(convertedCidr).toBe(originalCidr);
    });
  });

  it("should convert IP to binary and validate network calculations", () => {
    const testCases = [
      {
        ip: "192.168.1.100",
        subnet: "255.255.255.0",
        cidr: 24,
        expectedNetwork: "192.168.1.0",
      },
      {
        ip: "10.0.15.200",
        subnet: "255.255.0.0",
        cidr: 16,
        expectedNetwork: "10.0.0.0",
      },
      {
        ip: "172.16.5.10",
        subnet: "255.255.255.240",
        cidr: 28,
        expectedNetwork: "172.16.5.0",
      },
    ];

    testCases.forEach(({ ip, subnet, cidr, expectedNetwork }) => {
      const binaryString = ipToBinaryString(ip);
      expect(binaryString).toHaveLength(32);

      const networkLong = getNetworkAddress(ip, subnet);
      longToIp(networkLong);

      const convertedSubnet = cidrToSubnetMask(cidr);
      expect(convertedSubnet).toBe(subnet);

      expect(isInRange(ip, expectedNetwork, cidr)).toBe(true);
    });
  });

  it("should validate private IP ranges with conversions", () => {
    const ipRangeTests = [
      { ip: "192.168.1.1", isPrivate: true, class: "C" },
      { ip: "10.0.0.1", isPrivate: true, class: "A" },
      { ip: "172.16.1.1", isPrivate: true, class: "B" },
      { ip: "8.8.8.8", isPrivate: false, class: "A" },
      { ip: "1.1.1.1", isPrivate: false, class: "A" },
    ];

    ipRangeTests.forEach(({ ip, isPrivate, class: expectedClass }) => {
      expect(isPrivateIp(ip)).toBe(isPrivate);
      expect(getIpClass(ip)).toBe(expectedClass);

      const longValue = ipToLong(ip);
      const convertedIp = longToIp(longValue);
      expect(isPrivateIp(convertedIp)).toBe(isPrivate);
      expect(getIpClass(convertedIp)).toBe(expectedClass);
    });
  });

  it("should handle complex network range validations", () => {
    const networkTests = [
      {
        network: "192.168.1.0",
        cidr: 24,
        testIps: [
          { ip: "192.168.1.1", inRange: true },
          { ip: "192.168.1.255", inRange: true },
          { ip: "192.168.2.1", inRange: false },
          { ip: "192.168.0.255", inRange: false },
        ],
      },
      {
        network: "10.0.0.0",
        cidr: 8,
        testIps: [
          { ip: "10.255.255.255", inRange: true },
          { ip: "11.0.0.1", inRange: false },
          { ip: "10.192.168.1", inRange: true },
        ],
      },
    ];

    networkTests.forEach(({ network, cidr, testIps }) => {
      const subnetMask = cidrToSubnetMask(cidr);
      const convertedCidr = subnetMaskToCidr(subnetMask);
      expect(convertedCidr).toBe(cidr);

      testIps.forEach(({ ip, inRange }) => {
        expect(isInRange(ip, network, cidr)).toBe(inRange);

        const networkLong = getNetworkAddress(network, subnetMask);
        const networkIpFromLong = longToIp(networkLong);
        expect(isInRange(ip, networkIpFromLong, cidr)).toBe(inRange);
      });
    });
  });

  it("should perform binary string conversions with network analysis", () => {
    const testCases = [
      {
        ip: "192.168.1.1",
        expectedBinary: "11000000101010000000000100000001",
      },
      {
        ip: "255.255.255.255",
        expectedBinary: "11111111111111111111111111111111",
      },
      {
        ip: "0.0.0.0",
        expectedBinary: "00000000000000000000000000000000",
      },
    ];

    testCases.forEach(({ ip, expectedBinary }) => {
      const binaryString = ipToBinaryString(ip);
      expect(binaryString).toBe(expectedBinary);

      const longValue = ipToLong(ip);
      const binaryFromLong = longValue.toString(2).padStart(32, "0");
      expect(binaryFromLong).toBe(expectedBinary);

      const ipFromLong = longToIp(longValue);
      expect(ipFromLong).toBe(ip);
    });
  });

  it("should handle subnet calculations with multiple conversion methods", () => {
    const subnets = [
      {
        network: "192.168.1.0",
        cidr: 26,
        expectedMask: "255.255.255.192",
        expectedHosts: 62,
      },
      {
        network: "10.0.0.0",
        cidr: 12,
        expectedMask: "255.240.0.0",
        expectedHosts: 1048574,
      },
    ];

    subnets.forEach(({ network, cidr, expectedMask }) => {
      const mask = cidrToSubnetMask(cidr);
      expect(mask).toBe(expectedMask);

      const convertedCidr = subnetMaskToCidr(mask);
      expect(convertedCidr).toBe(cidr);

      const networkLong = getNetworkAddress(network, mask);
      const networkFromLong = longToIp(networkLong);

      expect(isInRange(network, networkFromLong, cidr)).toBe(true);

      const binaryNetwork = ipToBinaryString(networkFromLong);
      const binaryMask = ipToBinaryString(mask);

      expect(binaryNetwork).toHaveLength(32);
      expect(binaryMask).toHaveLength(32);

      const maskOnes = binaryMask.split("1").length - 1;
      expect(maskOnes).toBe(cidr);
    });
  });

  it("should validate edge cases in IP conversions", () => {
    const edgeCases = [
      { ip: "127.0.0.1", class: "A", isPrivate: false },
      { ip: "169.254.1.1", class: "B", isPrivate: false },
      { ip: "192.168.0.1", class: "C", isPrivate: true },
    ];

    edgeCases.forEach(({ ip, class: expectedClass, isPrivate }) => {
      const longValue = ipToLong(ip);
      const binary = ipToBinaryString(ip);
      const reconstructed = longToIp(longValue);

      expect(reconstructed).toBe(ip);
      expect(getIpClass(reconstructed)).toBe(expectedClass);
      expect(isPrivateIp(reconstructed)).toBe(isPrivate);

      expect(binary).toHaveLength(32);
      expect(parseInt(binary, 2)).toBe(longValue);
    });
  });
});

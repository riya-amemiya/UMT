import { getIpClass } from "@/IP/getIpClass";

describe("getIpClass", () => {
  describe("valid IP addresses", () => {
    test.each([
      // [IP address, expected class]
      ["1.0.0.0", "A"], // Start of class A
      ["126.0.0.1", "A"], // End of class A
      ["128.0.0.1", "B"], // Start of class B
      ["191.255.0.1", "B"], // End of class B
      ["192.0.0.1", "C"], // Start of class C
      ["223.255.0.1", "C"], // End of class C
      ["224.0.0.1", "D"], // Start of class D
      ["239.0.0.1", "D"], // End of class D
      ["240.0.0.1", "E"], // Start of class E
      ["255.255.255.255", "E"], // End of class E
    ])("should identify %s as class %s", (ip, expectedClass) => {
      expect(getIpClass(ip)).toBe(expectedClass);
    });
  });

  describe("invalid IP addresses", () => {
    test.each([
      ["", "empty string"],
      ["0.0.0.0", "reserved address"],
      ["256.0.0.1", "first octet > 255"],
      ["192.168", "incomplete IP"],
      ["192.168.1.1.1", "too many octets"],
      ["a.b.c.d", "non-numeric octets"],
      ["192.168.1", "too few octets"],
      ["-1.0.0.0", "negative octet"],
      ["1.2.3.4.5", "too many segments"],
    ])("should return empty string for %s (%s)", (ip) => {
      expect(getIpClass(ip)).toBe("");
    });
  });
});

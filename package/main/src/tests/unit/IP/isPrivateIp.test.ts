import { isPrivateIp } from "@/IP/isPrivateIp";
import { isInRange } from "@/IP/isInRange";

// Mock isInRange for specific test case
jest.mock("@/IP/isInRange");
const mockedIsInRange = isInRange as jest.MockedFunction<typeof isInRange>;

describe("isPrivateIp", () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockedIsInRange.mockImplementation((ip, network, cidr) => {
      // Default implementation using the real function
      const original = jest.requireActual("@/IP/isInRange");
      return original.isInRange(ip, network, cidr);
    });
  });

  describe("private IP addresses", () => {
    test.each([
      // [IP address, description]
      ["10.0.0.1", "Class A private - start range"],
      ["10.255.255.255", "Class A private - end range"],
      ["172.16.0.1", "Class B private - start range"],
      ["172.31.255.255", "Class B private - end range"],
      ["192.168.0.1", "Class C private - start range"],
      ["192.168.255.255", "Class C private - end range"],
    ])("should identify %s as private IP (%s)", (ip) => {
      expect(isPrivateIp(ip)).toBe(true);
    });
  });

  describe("non-private IP addresses", () => {
    test.each([
      // [IP address, description]
      ["9.255.255.255", "Just before Class A private"],
      ["11.0.0.0", "Just after Class A private"],
      ["172.15.255.255", "Just before Class B private"],
      ["172.32.0.0", "Just after Class B private"],
      ["192.167.255.255", "Just before Class C private"],
      ["192.169.0.0", "Just after Class C private"],
      ["8.8.8.8", "Google DNS"],
      ["1.1.1.1", "Cloudflare DNS"],
      ["169.254.0.1", "Link-local address"],
      ["127.0.0.1", "Localhost"],
    ])("should identify %s as non-private IP (%s)", (ip) => {
      expect(isPrivateIp(ip)).toBe(false);
    });
  });

  describe("invalid inputs", () => {
    test.each([
      [undefined, "IP address is required"],
      ["", "IP address is required"],
      ["256.256.256.256", "Invalid IP address"],
      ["192.168", "Invalid IP address"],
      ["a.b.c.d", "Invalid IP address"],
      ["192.168.1.1.1", "Invalid IP address"],
      ["-1.0.0.0", "Invalid IP address"],
    ])("should throw error for %s: %s", (ip, expectedError) => {
      expect(() => isPrivateIp(ip as string)).toThrow(expectedError);
    });

    test("should handle non-Error exceptions", () => {
      mockedIsInRange.mockImplementationOnce(() => {
        throw "Unexpected error"; // Throwing a string instead of Error
      });

      expect(() => isPrivateIp("192.168.1.1")).toThrow(
        "Invalid IP address: unknown error",
      );
    });
  });
});

import { isPrivateIp } from "@/IP/isPrivateIp";
describe("isPrivateIp", () => {
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
});

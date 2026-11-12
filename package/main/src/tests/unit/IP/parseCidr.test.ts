import { parseCidr } from "@/IP/parseCidr";

describe("parseCidr", () => {
  test.each([
    ["192.168.0.0/24", "192.168.0.0", 24],
    ["192.168.1.130/25", "192.168.1.128", 25],
    ["10.1.2.3/8", "10.0.0.0", 8],
    ["172.31.255.255/12", "172.16.0.0", 12],
    ["255.255.255.255/1", "128.0.0.0", 1],
    ["10.0.0.1/0", "0.0.0.0", 0],
    ["10.0.0.1/32", "10.0.0.1", 32],
  ])("parses %s", (cidr, networkAddress, prefixLength) => {
    expect(parseCidr(cidr)).toEqual({ networkAddress, prefixLength });
  });
});

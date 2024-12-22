import { isPrivateIp } from "@/IP/isPrivateIp";

describe("isPrivateIp", () => {
  it("10.0.0.1はプライベートIPである", () => {
    expect(isPrivateIp("10.0.0.1")).toBeTruthy();
  });

  it("172.16.0.1はプライベートIPである", () => {
    expect(isPrivateIp("172.16.0.1")).toBeTruthy();
  });

  it("192.168.0.1はプライベートIPである", () => {
    expect(isPrivateIp("192.168.0.1")).toBeTruthy();
  });

  it("8.8.8.8はプライベートIPではない", () => {
    expect(isPrivateIp("8.8.8.8")).toBeFalsy();
  });

  it("169.254.0.1はプライベートIPではない（リンクローカルアドレス）", () => {
    expect(isPrivateIp("169.254.0.1")).toBeFalsy();
  });
});

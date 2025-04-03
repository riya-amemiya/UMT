import { randomString } from "@/String/randomString";

describe("randomString", () => {
  it("should generate random string with default characters and length", () => {
    const str = randomString();
    expect(str).toHaveLength(8);
    expect(str).toMatch(/^[0-9A-Za-z]{8}$/);
  });

  it("should generate random string using custom character set", () => {
    const chars = "abc123";
    const str = randomString(10, chars);
    expect(str).toHaveLength(10);
    expect(str).toMatch(/^[abc123]{10}$/);
  });

  it("should generate random string with specified length", () => {
    const size = 20;
    const str = randomString(size, undefined);
    expect(str).toHaveLength(size);
  });
});

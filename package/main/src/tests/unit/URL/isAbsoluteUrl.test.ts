import { isAbsoluteUrl } from "@/URL/isAbsoluteUrl";

describe("isAbsoluteUrl", () => {
  it("should return true for http URLs", () => {
    expect(isAbsoluteUrl("http://example.com")).toBe(true);
    expect(isAbsoluteUrl("https://example.com")).toBe(true);
  });

  it("should return true for other schemes", () => {
    expect(isAbsoluteUrl("ftp://files.example")).toBe(true);
    expect(isAbsoluteUrl("mailto:user@host")).toBe(true);
    expect(isAbsoluteUrl("tel:+1234567890")).toBe(true);
    expect(isAbsoluteUrl("ssh://host")).toBe(true);
  });

  it("should return true for custom schemes", () => {
    expect(isAbsoluteUrl("custom+scheme://path")).toBe(true);
    expect(isAbsoluteUrl("my-app://deep-link")).toBe(true);
    expect(isAbsoluteUrl("x.y://test")).toBe(true);
  });

  it("should return false for relative paths", () => {
    expect(isAbsoluteUrl("/path/to/page")).toBe(false);
    expect(isAbsoluteUrl("relative/path")).toBe(false);
    expect(isAbsoluteUrl("./relative")).toBe(false);
    expect(isAbsoluteUrl("../parent")).toBe(false);
  });

  it("should return false for protocol-relative URLs", () => {
    expect(isAbsoluteUrl("//example.com")).toBe(false);
  });

  it("should return false for empty string", () => {
    expect(isAbsoluteUrl("")).toBe(false);
  });

  it("should return false for invalid scheme starts", () => {
    expect(isAbsoluteUrl("123://invalid")).toBe(false);
    expect(isAbsoluteUrl("+bad://invalid")).toBe(false);
  });
});

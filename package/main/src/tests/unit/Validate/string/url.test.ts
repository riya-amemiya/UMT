import { string, url } from "@/Validate/string";

describe("url", () => {
  it("should accept absolute URLs", () => {
    const validator = string([url()]);
    expect(validator("https://example.com").validate).toBe(true);
    expect(validator("http://localhost:3000/path?q=1").validate).toBe(true);
    expect(validator("ftp://files.example").validate).toBe(true);
    expect(validator("mailto:user@host").validate).toBe(true);
    expect(validator("HTTPS://EXAMPLE.COM").validate).toBe(true);
  });

  it("should reject relative or malformed URLs", () => {
    const validator = string([url("must be a URL")]);
    expect(validator("/path/to/page")).toEqual({
      validate: false,
      message: "must be a URL",
      type: "string",
    });
    expect(validator("relative/path").validate).toBe(false);
    expect(validator("").validate).toBe(false);
    expect(validator("1http://example.com").validate).toBe(false);
    expect(validator("//example.com").validate).toBe(false);
  });

  it("should return an empty message when no message is provided", () => {
    const validator = string([url()]);
    expect(validator("example.com").message).toBe("");
    expect(url().type).toBe("string");
  });
});

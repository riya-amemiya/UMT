import { stripTags } from "@/String/stripTags";

describe("stripTags", () => {
  it("should remove simple tags", () => {
    expect(stripTags("<p>Hello</p>")).toBe("Hello");
  });

  it("should remove nested tags but keep text", () => {
    expect(stripTags("<p>Hello <b>World</b></p>")).toBe("Hello World");
  });

  it("should remove tags with attributes", () => {
    expect(stripTags('<a href="https://example.com">link</a>')).toBe("link");
  });

  it("should remove self-closing tags", () => {
    expect(stripTags("line1<br/>line2")).toBe("line1line2");
  });

  it("should remove nested tag injections that survive a single pass", () => {
    expect(stripTags("<sc<script>ript>")).toBe("");
  });

  it("should leave text without tags unchanged", () => {
    expect(stripTags("plain text")).toBe("plain text");
  });

  it("should handle empty string", () => {
    expect(stripTags("")).toBe("");
  });
});

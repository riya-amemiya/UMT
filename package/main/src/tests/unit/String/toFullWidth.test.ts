import { toFullWidth } from "@/String/toFullWidth";

describe("toFullWidth", () => {
  it("should convert half-width digits to full-width", () => {
    expect(toFullWidth("123")).toBe("１２３");
  });

  it("should convert half-width uppercase letters to full-width", () => {
    expect(toFullWidth("ABC")).toBe("ＡＢＣ");
  });

  it("should convert half-width lowercase letters to full-width", () => {
    expect(toFullWidth("abc")).toBe("ａｂｃ");
  });

  it("should convert mixed alphanumeric characters", () => {
    expect(toFullWidth("Abc123")).toBe("Ａｂｃ１２３");
  });

  it("should leave non-alphanumeric characters unchanged", () => {
    expect(toFullWidth("a-b!")).toBe("ａ-ｂ!");
  });

  it("should handle empty string", () => {
    expect(toFullWidth("")).toBe("");
  });
});

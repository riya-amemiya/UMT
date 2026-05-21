import { deburr } from "@/String/deburr";

describe("deburr", () => {
  it("should remove acute accents", () => {
    expect(deburr("déjà vu")).toBe("deja vu");
  });

  it("should remove multiple diacritical marks", () => {
    expect(deburr("Crème brûlée")).toBe("Creme brulee");
  });

  it("should handle a mix of accented and plain characters", () => {
    expect(deburr("résumé café")).toBe("resume cafe");
  });

  it("should leave plain ASCII unchanged", () => {
    expect(deburr("hello world")).toBe("hello world");
  });

  it("should handle empty string", () => {
    expect(deburr("")).toBe("");
  });
});

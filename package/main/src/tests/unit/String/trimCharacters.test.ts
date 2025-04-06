import { trimCharacters } from "@/String/trimCharacters";

describe("trimCharacters", () => {
  it("should remove specified characters from both ends of string", () => {
    expect(trimCharacters("---Hello World---", "-")).toBe("Hello World");
  });

  it("should not remove specified characters from middle of string", () => {
    expect(trimCharacters("---Hello-World---", "-")).toBe("Hello-World");
  });

  it("should return empty string when input is empty", () => {
    expect(trimCharacters("", "-")).toBe("");
  });

  it("should return original string when no characters are specified to remove", () => {
    expect(trimCharacters("Hello World", "")).toBe("Hello World");
  });
});

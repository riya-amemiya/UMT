import { randomStringInitialization } from "@/String/randomStringInitialization";

describe("randomStringInitialization", () => {
  it("should generate random strings with default character set", () => {
    const randomStringFunc = randomStringInitialization();
    const result = randomStringFunc(10);
    expect(result).toHaveLength(10);
  });

  it("should generate random strings with custom character set", () => {
    const customCharSet = "ABC123";
    const randomStringFunc = randomStringInitialization(customCharSet);
    const result = randomStringFunc(5);
    expect(result).toHaveLength(5);
    expect([...result].every((char) => customCharSet.includes(char))).toBe(
      true,
    );
  });
});

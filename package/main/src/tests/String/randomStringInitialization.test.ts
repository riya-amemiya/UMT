import { randomStringInitialization } from "@/String/randomStringInitialization";

describe("randomStringInitialization", () => {
  it("デフォルトの文字セットでランダムな文字列を生成する", () => {
    const randomStringFunc = randomStringInitialization();
    const result = randomStringFunc(10);
    expect(result).toHaveLength(10);
  });

  it("カスタムの文字セットでランダムな文字列を生成する", () => {
    const customCharSet = "ABC123";
    const randomStringFunc = randomStringInitialization(customCharSet);
    const result = randomStringFunc(5);
    expect(result).toHaveLength(5);
    expect([...result].every((char) => customCharSet.includes(char))).toBe(
      true,
    );
  });
});

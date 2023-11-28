import { randomString } from "@/String/randomString";

describe("randomString", () => {
  it("デフォルトの文字列と長さでランダムな文字列を生成する", () => {
    const str = randomString();
    expect(str).toHaveLength(8);
    expect(str).toMatch(/^[0-9A-Za-z]{8}$/);
  });

  it("カスタムの文字列セットを使用してランダムな文字列を生成する", () => {
    const chars = "abc123";
    const str = randomString(chars, 10);
    expect(str).toHaveLength(10);
    expect(str).toMatch(/^[abc123]{10}$/);
  });

  it("指定された長さでランダムな文字列を生成する", () => {
    const size = 20;
    const str = randomString(undefined, size);
    expect(str).toHaveLength(size);
  });
});

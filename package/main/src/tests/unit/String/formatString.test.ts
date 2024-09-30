import { formatString } from "@/String/formatString";

describe("formatString", () => {
  it("指定された値でプレースホルダーを置換する", () => {
    const template = "こんにちは、{0}！今日は{1}ですね。";
    const result = formatString(template, "世界", "晴れ");
    expect(result).toBe("こんにちは、世界！今日は晴れですね。");
  });

  it("値が未定義のプレースホルダーは置換されない", () => {
    const template = "こんにちは、{0}！{1}はどうですか？";
    const result = formatString(template, "世界");
    expect(result).toBe("こんにちは、世界！{1}はどうですか？");
  });

  it("存在しないインデックスのプレースホルダーは置換されない", () => {
    const template = "こんにちは、{0}！{2}はありません。";
    const result = formatString(template, "世界", "晴れ");
    expect(result).toBe("こんにちは、世界！{2}はありません。");
  });
});

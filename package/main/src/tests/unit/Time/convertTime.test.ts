import { convertTime } from "@/Time/convertTime";

describe("convertTime", () => {
  // 正常系のテスト
  test("1時間を秒に変換", () => {
    expect(convertTime("1", "hours", "seconds")).toBe(3600);
  });

  test("3600秒を時間に変換", () => {
    expect(convertTime("3600", "seconds", "hours")).toBe(1);
  });

  test("90分を時間に変換", () => {
    expect(convertTime("90", "minutes", "hours")).toBe(1.5);
  });

  test("1時間をミリ秒に変換", () => {
    expect(convertTime("1", "hours", "milliseconds")).toBe(3600000);
  });

  test("0.5秒をミリ秒に変換", () => {
    expect(convertTime("0.5", "seconds", "milliseconds")).toBe(500);
  });

  test("1000ミリ秒を秒に変換", () => {
    expect(convertTime("1000", "milliseconds", "seconds")).toBe(1);
  });

  // 同じ単位間の変換
  test("同じ単位（秒から秒）への変換", () => {
    expect(convertTime("10", "seconds", "seconds")).toBe(10);
  });

  // 小数点を含む入力
  test("小数点を含む入力（1.5時間を分に変換）", () => {
    expect(convertTime("1.5", "hours", "minutes")).toBe(90);
  });

  // 境界値テスト
  test("0の変換", () => {
    expect(convertTime("0", "hours", "seconds")).toBe(0);
  });

  test("非常に大きな数値の変換", () => {
    expect(convertTime("1e9", "milliseconds", "hours")).toBe(
      1000000000 / (60 * 60 * 1000),
    );
  });
});

import { mathConverter } from "@/Math/mathConverter";

describe("mathConverter関数のテスト", () => {
  // 乗算のテスト
  test("乗算の変換", () => {
    expect(mathConverter("1250*1250")).toBe("1500*1000+400*100+200*100+50*50");
    expect(mathConverter("1350*1350")).toBe(
      "1700*1000+600*100+400*100+200*100+50*50",
    );
    expect(mathConverter("1550*1550")).toBe(
      "2100*1000+1000*100+800*100+600*100+400*100+200*100+50*50",
    );
  });

  // 累乗のテスト
  test("累乗の変換", () => {
    expect(mathConverter("1250^2")).toBe("1500*1000+400*100+200*100+50*50");
    expect(mathConverter("1350^2")).toBe(
      "1700*1000+600*100+400*100+200*100+50*50",
    );
  });

  // 何も変換しない場合のテスト
  test("変換が不要な場合", () => {
    expect(mathConverter("1250")).toBe("1250");
  });

  // 複数のオペランドがある場合のテスト
  test("複数のオペランドがある場合の変換", () => {
    expect(mathConverter("1250+1250")).toBe("1250+1250");
  });

  // 無効な入力のテスト
  test("無効な入力の場合", () => {
    expect(mathConverter("abc")).toBe("abc");
  });

  // primaryがない場合のテスト
  test("primaryがない場合", () => {
    expect(mathConverter("0*0")).toBe("0*0");
  });
});

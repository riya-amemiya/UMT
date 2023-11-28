import {
  OneSecondMs,
  OneMinuteMs,
  OneHourMs,
  OneDayMs,
  OneWeekMs,
  OneMonthMs,
  OneYearMs,
  OneMonthMs28,
  OneMonthMs29,
  OneMonthMs31,
  OneYearMs366,
} from "@/Consts";

describe("時間定数のテスト", () => {
  it("1秒は1000ミリ秒", () => {
    expect(OneSecondMs).toBe(1000);
  });

  it("1分は60000ミリ秒", () => {
    expect(OneMinuteMs).toBe(60000);
  });

  it("1時間は3600000ミリ秒", () => {
    expect(OneHourMs).toBe(3600000);
  });

  it("1日は86400000ミリ秒", () => {
    expect(OneDayMs).toBe(86400000);
  });

  it("1週間は604800000ミリ秒", () => {
    expect(OneWeekMs).toBe(604800000);
  });

  it("1ヶ月（28日として）は2419200000ミリ秒", () => {
    expect(OneMonthMs28).toBe(2419200000);
  });

  it("1ヶ月（29日として）は2505600000ミリ秒", () => {
    expect(OneMonthMs29).toBe(2505600000);
  });

  it("1ヶ月（30日として）は2592000000ミリ秒", () => {
    expect(OneMonthMs).toBe(2592000000);
  });

  it("1ヶ月（31日として）は2678400000ミリ秒", () => {
    expect(OneMonthMs31).toBe(2678400000);
  });

  it("1年（365日として）は31536000000ミリ秒", () => {
    expect(OneYearMs).toBe(31536000000);
  });

  it("1年（366日として）は31622400000ミリ秒", () => {
    expect(OneYearMs366).toBe(31622400000);
  });
});

import {
  OneDayMs,
  OneHourMs,
  OneMinuteMs,
  OneMonthMs,
  OneMonthMs28,
  OneMonthMs29,
  OneMonthMs31,
  OneSecondMs,
  OneWeekMs,
  OneYearMs,
  OneYearMs366,
} from "@/Consts/clock";

describe("Time constants", () => {
  describe("Basic time unit values", () => {
    it("should define one second as 1000 milliseconds", () => {
      expect(OneSecondMs).toBe(1000);
    });

    it("should define one minute as 60000 milliseconds", () => {
      expect(OneMinuteMs).toBe(60_000);
    });

    it("should define one hour as 3600000 milliseconds", () => {
      expect(OneHourMs).toBe(3_600_000);
    });

    it("should define one day as 86400000 milliseconds", () => {
      expect(OneDayMs).toBe(86_400_000);
    });

    it("should define one week as 604800000 milliseconds", () => {
      expect(OneWeekMs).toBe(604_800_000);
    });
  });

  describe("Time unit relationships", () => {
    it("should maintain correct relationships between units", () => {
      expect(OneMinuteMs).toBe(OneSecondMs * 60);
      expect(OneHourMs).toBe(OneMinuteMs * 60);
      expect(OneDayMs).toBe(OneHourMs * 24);
      expect(OneWeekMs).toBe(OneDayMs * 7);
    });
  });

  describe("Month variations", () => {
    it("should define correct relationships for different month lengths", () => {
      expect(OneMonthMs28).toBe(OneDayMs * 28);
      expect(OneMonthMs29).toBe(OneDayMs * 29);
      expect(OneMonthMs).toBe(OneDayMs * 30);
      expect(OneMonthMs31).toBe(OneDayMs * 31);
    });

    it("should maintain correct order of month lengths", () => {
      expect(OneMonthMs28).toBeLessThan(OneMonthMs29);
      expect(OneMonthMs29).toBeLessThan(OneMonthMs);
      expect(OneMonthMs).toBeLessThan(OneMonthMs31);
    });
  });

  describe("Year variations", () => {
    it("should define correct relationships for different year lengths", () => {
      expect(OneYearMs).toBe(OneDayMs * 365);
      expect(OneYearMs366).toBe(OneDayMs * 366);
    });

    it("should maintain correct relationship between regular and leap years", () => {
      expect(OneYearMs).toBeLessThan(OneYearMs366);
      expect(OneYearMs366 - OneYearMs).toBe(OneDayMs);
    });
  });

  it("should define one month (28 days) as 2419200000 milliseconds", () => {
    expect(OneMonthMs28).toBe(2_419_200_000);
  });

  it("should define one month (29 days) as 2505600000 milliseconds", () => {
    expect(OneMonthMs29).toBe(2_505_600_000);
  });

  it("should define one month (30 days) as 2592000000 milliseconds", () => {
    expect(OneMonthMs).toBe(2_592_000_000);
  });

  it("should define one month (31 days) as 2678400000 milliseconds", () => {
    expect(OneMonthMs31).toBe(2_678_400_000);
  });

  it("should define one year (365 days) as 31536000000 milliseconds", () => {
    expect(OneYearMs).toBe(31_536_000_000);
  });

  it("should define one year (366 days) as 31622400000 milliseconds", () => {
    expect(OneYearMs366).toBe(31_622_400_000);
  });
});

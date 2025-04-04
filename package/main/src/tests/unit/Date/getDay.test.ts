import { getDay } from "@/Date/getDay";

describe("getDay", () => {
  it("should return Sunday in different languages", () => {
    expect(getDay(0, "en")).toBe("Sun");
    expect(getDay(0, "ja")).toBe("日");
    expect(getDay(0, "ko")).toBe("일");
    expect(getDay(0, "de")).toBe("So");
    expect(getDay(0, "fr")).toBe("Dim");
  });

  it("should return weekdays in different languages", () => {
    // Test Wednesday (3) in different languages
    expect(getDay(3, "en")).toBe("Wed");
    expect(getDay(3, "ja")).toBe("水");
    expect(getDay(3, "ko")).toBe("수");
    expect(getDay(3, "de")).toBe("Mi");
    expect(getDay(3, "fr")).toBe("Mer");
  });

  it("should use Japanese as default language", () => {
    expect(getDay(0)).toBe("日");
    expect(getDay(1)).toBe("月");
    expect(getDay(2)).toBe("火");
    expect(getDay(3)).toBe("水");
    expect(getDay(4)).toBe("木");
    expect(getDay(5)).toBe("金");
    expect(getDay(6)).toBe("土");
  });

  it("should return Sunday for invalid day numbers", () => {
    expect(getDay(-1)).toBe("日");
    expect(getDay(7)).toBe("日");
    expect(getDay(100)).toBe("日");
  });

  it("should handle all days in English", () => {
    const days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    days.forEach((day, index) => {
      expect(getDay(index, "en")).toBe(day);
    });
  });
});

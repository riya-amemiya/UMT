import { format } from "@/Date/format";

describe("format", () => {
  it("should format date correctly", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date, "YYYY-MM-DD")).toBe("2023-06-10");
    expect(format(date, "YYYY/MM/DD HH:mm:ss")).toBe("2023/06/10 15:30:45");
    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS")).toBe(
      "2023-06-10 15:30:45.123",
    );
    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS Z")).toBe(
      "2023-06-10 15:30:45.123 +09:00",
    );
    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS ZZ")).toBe(
      "2023-06-10 15:30:45.123 +0900",
    );
  });

  it("should use default format if format string is not provided", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date)).toBe("2023-06-10T15:30:45+09:00");
  });

  it("should handle escaped characters", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date, "[Year:] YYYY [Month:] MM [Day:] DD")).toBe(
      "Year: 2023 Month: 06 Day: 10",
    );
  });

  it("should handle different formats", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date, "YY-M-D")).toBe("23-6-10");
    expect(format(date, "hh:mm:ss A")).toBe("03:30:45 PM");
    expect(format(date, "h:m:s a")).toBe("3:30:45 pm");
  });
});

import { format } from "@/Date/format";
import { getTimezoneOffsetString } from "@/Date/getTimezoneOffsetString";

describe("format", () => {
  it("should format date correctly", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date, "YYYY-MM-DD")).toBe("2023-06-10");
    expect(format(date, "YYYY/MM/DD HH:mm:ss")).toBe("2023/06/10 15:30:45");
    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS")).toBe(
      "2023-06-10 15:30:45.123",
    );

    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS Z")).toBe(
      `2023-06-10 15:30:45.123 ${getTimezoneOffsetString(date)}`,
    );
    expect(format(date, "YYYY-MM-DD HH:mm:ss.SSS ZZ")).toBe(
      `2023-06-10 15:30:45.123 ${getTimezoneOffsetString(date).replace(
        ":",
        "",
      )}`,
    );
  });

  it("should use default format if format string is not provided", () => {
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(format(date)).toBe(
      `2023-06-10T15:30:45${getTimezoneOffsetString(date)}`,
    );
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

  it("should handle day of week format", () => {
    const date = new Date(2023, 5, 10); // June 10, 2023 was a Saturday
    expect(format(date, "d")).toBe("6");
    expect(format(date, "YYYY-MM-DD (d)")).toBe("2023-06-10 (6)");
  });

  it("should handle morning hours correctly", () => {
    const date = new Date(2023, 5, 10, 9, 5, 8, 4);
    expect(format(date, "HH:mm:ss")).toBe("09:05:08");
    expect(format(date, "H:m:s")).toBe("9:5:8");
    expect(format(date, "hh:mm A")).toBe("09:05 AM");
    expect(format(date, "h:mm a")).toBe("9:05 am");
  });

  it("should handle invalid date", () => {
    // @ts-expect-error Testing invalid input
    expect(() => format("not a date")).toThrow("Invalid Date in format");
    // @ts-expect-error Testing invalid input
    expect(() => format(null)).toThrow("Invalid Date in format");
  });
});

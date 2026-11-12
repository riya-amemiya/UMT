import { format } from "@/Date/format";
import { parseDate } from "@/Date/parseDate";

const isInvalid = (date: Date): boolean => Number.isNaN(date.getTime());

describe("parseDate", () => {
  it("parses a date in local time", () => {
    expect(parseDate("2025-04-04", "YYYY-MM-DD").getTime()).toBe(
      new Date(2025, 3, 4).getTime(),
    );
  });

  it("parses a full date-time with milliseconds", () => {
    expect(
      parseDate("2023/06/10 15:30:45.123", "YYYY/MM/DD HH:mm:ss.SSS").getTime(),
    ).toBe(new Date(2023, 5, 10, 15, 30, 45, 123).getTime());
  });

  it("parses tokens without leading zeros", () => {
    expect(parseDate("23-6-1 5:7:9", "YY-M-D H:m:s").getTime()).toBe(
      new Date(2023, 5, 1, 5, 7, 9).getTime(),
    );
  });

  it("reads YY as 2000-2099", () => {
    expect(parseDate("99", "YY").getFullYear()).toBe(2099);
    expect(parseDate("00", "YY").getFullYear()).toBe(2000);
  });

  it("defaults missing fields to 1970-01-01 00:00:00.000", () => {
    expect(parseDate("10:20", "HH:mm").getTime()).toBe(
      new Date(1970, 0, 1, 10, 20).getTime(),
    );
    expect(parseDate("", "").getTime()).toBe(new Date(1970, 0, 1).getTime());
  });

  it("keeps years below 100 as written", () => {
    const date = parseDate("0050-03-01", "YYYY-MM-DD");
    expect(date.getFullYear()).toBe(50);
    expect(date.getMonth()).toBe(2);
    expect(date.getDate()).toBe(1);
  });

  it("converts 12-hour clock values with a meridiem", () => {
    expect(parseDate("3:30 PM", "h:mm A").getHours()).toBe(15);
    expect(parseDate("03:30 am", "hh:mm a").getHours()).toBe(3);
    expect(parseDate("12:00 AM", "hh:mm A").getHours()).toBe(0);
    expect(parseDate("12:00 pm", "hh:mm a").getHours()).toBe(12);
  });

  it("uses 12-hour values as written without a meridiem", () => {
    expect(parseDate("12", "hh").getHours()).toBe(12);
    expect(parseDate("7", "h").getHours()).toBe(7);
  });

  it("ignores the meridiem for 24-hour tokens", () => {
    expect(parseDate("15 AM", "HH A").getHours()).toBe(15);
  });

  it("applies the Z offset", () => {
    expect(
      parseDate("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ").getTime(),
    ).toBe(Date.UTC(2025, 3, 4, 6, 30));
    expect(
      parseDate("2025-04-04 01:15 -05:30", "YYYY-MM-DD HH:mm Z").getTime(),
    ).toBe(Date.UTC(2025, 3, 4, 6, 45));
  });

  it("applies the ZZ offset", () => {
    expect(
      parseDate("2025-04-04 00:00 +0000", "YYYY-MM-DD HH:mm ZZ").getTime(),
    ).toBe(Date.UTC(2025, 3, 4));
    expect(
      parseDate("2025-01-01 00:00 +0100", "YYYY-MM-DD HH:mm ZZ").getTime(),
    ).toBe(Date.UTC(2024, 11, 31, 23));
  });

  it("round-trips with format", () => {
    const pattern = "YYYY-MM-DD HH:mm:ss.SSS Z";
    const date = new Date(2023, 5, 10, 15, 30, 45, 123);
    expect(parseDate(format(date, pattern), pattern).getTime()).toBe(
      date.getTime(),
    );
  });

  it("matches escaped and literal text", () => {
    expect(parseDate("Year: 2023 (06)", "[Year:] YYYY (MM)").getTime()).toBe(
      new Date(2023, 5, 1).getTime(),
    );
    expect(parseDate("2023.06.10", "YYYY.MM.DD").getTime()).toBe(
      new Date(2023, 5, 10).getTime(),
    );
    expect(isInvalid(parseDate("2023x06x10", "YYYY.MM.DD"))).toBe(true);
  });

  it("accepts a day of week that agrees with the date", () => {
    expect(parseDate("2025-04-21 1", "YYYY-MM-DD d").getTime()).toBe(
      new Date(2025, 3, 21).getTime(),
    );
  });

  it("rejects a day of week that disagrees with the date", () => {
    expect(isInvalid(parseDate("2025-04-21 2", "YYYY-MM-DD d"))).toBe(true);
  });

  it("lets the last occurrence of a token win", () => {
    expect(parseDate("2024 2025", "YYYY YYYY").getFullYear()).toBe(2025);
  });

  it("returns an Invalid Date when the string does not match", () => {
    expect(isInvalid(parseDate("2025-4-04", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("2025-04-04 extra", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("abcd-04-04", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("10:00 XM", "hh:mm A"))).toBe(true);
    expect(isInvalid(parseDate("10:00 PM", "hh:mm a"))).toBe(true);
  });

  it("rejects non-ASCII digits", () => {
    expect(
      isInvalid(
        parseDate(String.fromCodePoint(1634, 1632, 1634, 1637), "YYYY"),
      ),
    ).toBe(true);
  });

  it("returns an Invalid Date for out-of-range fields", () => {
    expect(isInvalid(parseDate("2025-13-01", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("2025-00-01", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("2025-04-00", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("2025-04-31", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("2025-02-29", "YYYY-MM-DD"))).toBe(true);
    expect(isInvalid(parseDate("24:00", "HH:mm"))).toBe(true);
    expect(isInvalid(parseDate("10:60", "HH:mm"))).toBe(true);
    expect(isInvalid(parseDate("10:00:60", "HH:mm:ss"))).toBe(true);
    expect(isInvalid(parseDate("00 AM", "hh A"))).toBe(true);
    expect(isInvalid(parseDate("13 PM", "hh A"))).toBe(true);
    expect(isInvalid(parseDate("10:00 +24:00", "HH:mm Z"))).toBe(true);
    expect(isInvalid(parseDate("10:00 +0960", "HH:mm ZZ"))).toBe(true);
  });

  it("accepts February 29 in a leap year", () => {
    expect(parseDate("2024-02-29", "YYYY-MM-DD").getTime()).toBe(
      new Date(2024, 1, 29).getTime(),
    );
  });

  it("treats an opening bracket inside escaped text as a literal", () => {
    expect(parseDate("[xy", "[x[y]").getTime()).toBe(
      new Date(1970, 0, 1).getTime(),
    );
    expect(isInvalid(parseDate("x[y", "[x[y]"))).toBe(true);
  });
});

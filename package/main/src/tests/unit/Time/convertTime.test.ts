import { convertTime } from "@/Time/convertTime";

describe("convertTime", () => {
  // long format to long format
  test("converts 1 hour to seconds", () => {
    expect(convertTime("1", "hours", "seconds")).toBe(3600);
    expect(convertTime(1, "hours", "seconds")).toBe(3600);
  });

  test("converts 3600 seconds to hours", () => {
    expect(convertTime("3600", "seconds", "hours")).toBe(1);
  });

  test("converts 90 minutes to hours", () => {
    expect(convertTime("90", "minutes", "hours")).toBe(1.5);
  });

  test("converts 1 hour to milliseconds", () => {
    expect(convertTime("1", "hours", "milliseconds")).toBe(3600000);
  });

  test("converts 0.5 seconds to milliseconds", () => {
    expect(convertTime("0.5", "seconds", "milliseconds")).toBe(500);
  });

  test("converts 1000 milliseconds to seconds", () => {
    expect(convertTime("1000", "milliseconds", "seconds")).toBe(1);
  });

  test("converts between same units (seconds to seconds)", () => {
    expect(convertTime("10", "seconds", "seconds")).toBe(10);
  });

  test("converts decimal input (1.5 hours to minutes)", () => {
    expect(convertTime("1.5", "hours", "minutes")).toBe(90);
  });

  test("converts zero", () => {
    expect(convertTime("0", "hours", "seconds")).toBe(0);
  });

  test("converts very large numbers", () => {
    expect(convertTime("1e9", "milliseconds", "hours")).toBe(
      1000000000 / (60 * 60 * 1000),
    );
  });

  // short format to short format
  test("converts 1 hour to seconds (short format)", () => {
    expect(convertTime("1", "h", "s")).toBe(3600);
  });

  test("converts 3600 seconds to hours (short format)", () => {
    expect(convertTime("3600", "s", "h")).toBe(1);
  });

  test("converts 90 minutes to hours (short format)", () => {
    expect(convertTime("90", "m", "h")).toBe(1.5);
  });

  test("converts 1 hour to milliseconds (short format)", () => {
    expect(convertTime("1", "h", "ms")).toBe(3600000);
  });

  test("converts 0.5 seconds to milliseconds (short format)", () => {
    expect(convertTime("0.5", "s", "ms")).toBe(500);
  });

  test("converts 1000 milliseconds to seconds (short format)", () => {
    expect(convertTime("1000", "ms", "s")).toBe(1);
  });

  test("converts between same units (seconds to seconds) (short format)", () => {
    expect(convertTime("10", "s", "s")).toBe(10);
  });

  test("converts decimal input (1.5 hours to minutes) (short format)", () => {
    expect(convertTime("1.5", "h", "m")).toBe(90);
  });

  test("converts zero (short format)", () => {
    expect(convertTime("0", "h", "s")).toBe(0);
  });

  test("converts very large numbers (short format)", () => {
    expect(convertTime("1e9", "ms", "h")).toBe(1000000000 / (60 * 60 * 1000));
  });

  // long format to short format

  test("converts 1 hour to seconds (long to short)", () => {
    expect(convertTime("1", "hours", "s")).toBe(3600);
  });

  test("converts 3600 seconds to hours (long to short)", () => {
    expect(convertTime("3600", "seconds", "h")).toBe(1);
  });

  test("converts 90 minutes to hours (long to short)", () => {
    expect(convertTime("90", "minutes", "h")).toBe(1.5);
  });

  test("converts 1 hour to milliseconds (long to short)", () => {
    expect(convertTime("1", "hours", "ms")).toBe(3600000);
  });

  test("converts 0.5 seconds to milliseconds (long to short)", () => {
    expect(convertTime("0.5", "seconds", "ms")).toBe(500);
  });

  test("converts 1000 milliseconds to seconds (long to short)", () => {
    expect(convertTime("1000", "milliseconds", "s")).toBe(1);
  });

  test("converts between same units (seconds to seconds) (long to short)", () => {
    expect(convertTime("10", "seconds", "s")).toBe(10);
  });

  test("converts decimal input (1.5 hours to minutes) (long to short)", () => {
    expect(convertTime("1.5", "hours", "m")).toBe(90);
  });

  test("converts zero (long to short)", () => {
    expect(convertTime("0", "hours", "s")).toBe(0);
  });

  test("converts very large numbers (long to short)", () => {
    expect(convertTime("1e9", "milliseconds", "h")).toBe(
      1000000000 / (60 * 60 * 1000),
    );
  });

  // short format to long format

  test("converts 1 hour to seconds (short to long)", () => {
    expect(convertTime("1", "h", "seconds")).toBe(3600);
  });

  test("converts 3600 seconds to hours (short to long)", () => {
    expect(convertTime("3600", "s", "hours")).toBe(1);
  });

  test("converts 90 minutes to hours (short to long)", () => {
    expect(convertTime("90", "m", "hours")).toBe(1.5);
  });

  test("converts 1 hour to milliseconds (short to long)", () => {
    expect(convertTime("1", "h", "milliseconds")).toBe(3600000);
  });

  test("converts 0.5 seconds to milliseconds (short to long)", () => {
    expect(convertTime("0.5", "s", "milliseconds")).toBe(500);
  });

  test("converts 1000 milliseconds to seconds (short to long)", () => {
    expect(convertTime("1000", "ms", "seconds")).toBe(1);
  });

  test("converts between same units (seconds to seconds) (short to long)", () => {
    expect(convertTime("10", "s", "seconds")).toBe(10);
  });

  test("converts decimal input (1.5 hours to minutes) (short to long)", () => {
    expect(convertTime("1.5", "h", "minutes")).toBe(90);
  });

  test("converts zero (short to long)", () => {
    expect(convertTime("0", "h", "seconds")).toBe(0);
  });

  test("converts very large numbers (short to long)", () => {
    expect(convertTime("1e9", "ms", "hours")).toBe(
      1000000000 / (60 * 60 * 1000),
    );
  });
});

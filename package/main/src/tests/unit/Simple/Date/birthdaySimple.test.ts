import { birthdaySimple } from "@/Simple/Date/birthdaySimple";

describe("birthdaySimple function", () => {
  // Mock current date to ensure consistent test results
  const mockDate = new Date("2024-06-13T12:00:00Z");
  beforeAll(() => {
    jest.useFakeTimers();
    jest.setSystemTime(mockDate);
  });

  afterAll(() => {
    jest.useRealTimers();
  });

  describe("string format with hyphen delimiter", () => {
    test("basic age calculation", () => {
      expect(birthdaySimple("2000-01-01")).toBe(24);
      expect(birthdaySimple("1990-06-13")).toBe(34);
      expect(birthdaySimple("2020-12-31")).toBe(3);
    });

    test("current year birth", () => {
      expect(birthdaySimple("2024-01-01")).toBe(0);
      expect(birthdaySimple("2024-06-01")).toBe(0);
    });

    test("future birthday", () => {
      expect(birthdaySimple("2024-12-31")).toBe(0);
      expect(birthdaySimple("2025-01-01")).toBe(0);
    });

    test("month and day edge cases", () => {
      expect(birthdaySimple("2000-02-29")).toBe(24); // leap year
      expect(birthdaySimple("2001-02-28")).toBe(23);
      expect(birthdaySimple("2000-12-31")).toBe(23);
    });
  });

  describe("string format with colon delimiter", () => {
    test("basic age calculation", () => {
      expect(birthdaySimple("2000:01:01")).toBe(24);
      expect(birthdaySimple("1995:06:13")).toBe(29);
      expect(birthdaySimple("2010:03:15")).toBe(14);
    });

    test("various month and day patterns", () => {
      expect(birthdaySimple("2000:02:29")).toBe(24);
      expect(birthdaySimple("2000:12:01")).toBe(23);
    });
  });

  describe("string format with slash delimiter", () => {
    test("basic age calculation", () => {
      expect(birthdaySimple("2000/01/01")).toBe(24);
      expect(birthdaySimple("1985/06/13")).toBe(39);
      expect(birthdaySimple("2015/09/20")).toBe(8);
    });

    test("various month and day patterns", () => {
      expect(birthdaySimple("2000/02/29")).toBe(24);
      expect(birthdaySimple("2000/11/30")).toBe(23);
    });
  });

  describe("Date object format", () => {
    test("basic age calculation", () => {
      expect(birthdaySimple(new Date(2000, 0, 1))).toBe(24); // Note: Date.getMonth() is 0-indexed
      expect(birthdaySimple(new Date(1990, 5, 13))).toBe(34); // June is 5
      expect(birthdaySimple(new Date(2020, 11, 31))).toBe(3); // December is 11
    });

    test("current year birth", () => {
      expect(birthdaySimple(new Date(2024, 0, 1))).toBe(0);
      expect(birthdaySimple(new Date(2024, 5, 1))).toBe(0);
    });

    test("leap year test", () => {
      expect(birthdaySimple(new Date(2000, 1, 29))).toBe(24); // February 29
      expect(birthdaySimple(new Date(2004, 1, 29))).toBe(20);
    });
  });

  describe("object format", () => {
    test("basic age calculation", () => {
      expect(birthdaySimple({ year: 2000, mon: 1, day: 1 })).toBe(24);
      expect(birthdaySimple({ year: 1990, mon: 6, day: 13 })).toBe(34);
      expect(birthdaySimple({ year: 2020, mon: 12, day: 31 })).toBe(3);
    });

    test("current year birth", () => {
      expect(birthdaySimple({ year: 2024, mon: 1, day: 1 })).toBe(0);
      expect(birthdaySimple({ year: 2024, mon: 6, day: 1 })).toBe(0);
    });

    test("month and day edge cases", () => {
      expect(birthdaySimple({ year: 2000, mon: 2, day: 29 })).toBe(24);
      expect(birthdaySimple({ year: 2000, mon: 12, day: 31 })).toBe(23);
    });
  });

  describe("timezone difference", () => {
    test("default timezone difference (9 hours)", () => {
      expect(birthdaySimple("2000-01-01")).toBe(24);
      expect(birthdaySimple("2000-01-01", 9)).toBe(24);
    });

    test("different timezone differences", () => {
      expect(birthdaySimple("2000-01-01", 0)).toBe(24); // UTC
      expect(birthdaySimple("2000-01-01", 5)).toBe(24); // positive value equivalent to EST
      expect(birthdaySimple("2000-01-01", 8)).toBe(24); // China Standard Time
    });

    test("timezone specification for each format", () => {
      expect(birthdaySimple("2000:01:01", 0)).toBe(24);
      expect(birthdaySimple("2000/01/01", 8)).toBe(24);
      expect(birthdaySimple(new Date(2000, 0, 1), 5)).toBe(24);
      expect(birthdaySimple({ year: 2000, mon: 1, day: 1 }, 3)).toBe(24);
    });
  });

  describe("boundary value tests", () => {
    test("very old birthdays", () => {
      expect(birthdaySimple("1900-01-01")).toBe(124);
      expect(birthdaySimple("1950-06-15")).toBe(73);
    });

    test("recent birthdays", () => {
      expect(birthdaySimple("2023-01-01")).toBe(1);
      expect(birthdaySimple("2024-01-01")).toBe(0);
    });

    test("month and day boundaries", () => {
      expect(birthdaySimple("2000-01-31")).toBe(24);
      expect(birthdaySimple("2000-03-31")).toBe(24);
      expect(birthdaySimple("2000-04-30")).toBe(24);
      expect(birthdaySimple("2000-05-31")).toBe(24);
    });
  });

  describe("special cases", () => {
    test("birthday today", () => {
      expect(birthdaySimple("2024-06-13")).toBe(0); // today is birthday
    });

    test("birthday tomorrow", () => {
      expect(birthdaySimple("2024-06-14")).toBe(0);
    });

    test("birthday yesterday", () => {
      expect(birthdaySimple("2024-06-12")).toBe(0);
    });
  });
});

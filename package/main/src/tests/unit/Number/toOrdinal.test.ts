import { toOrdinal } from "@/Number/toOrdinal";

describe("toOrdinal", () => {
  it("should return 'st' for numbers ending in 1", () => {
    expect(toOrdinal(1)).toBe("1st");
    expect(toOrdinal(21)).toBe("21st");
    expect(toOrdinal(31)).toBe("31st");
    expect(toOrdinal(101)).toBe("101st");
    expect(toOrdinal(121)).toBe("121st");
  });

  it("should return 'nd' for numbers ending in 2", () => {
    expect(toOrdinal(2)).toBe("2nd");
    expect(toOrdinal(22)).toBe("22nd");
    expect(toOrdinal(32)).toBe("32nd");
    expect(toOrdinal(102)).toBe("102nd");
    expect(toOrdinal(122)).toBe("122nd");
  });

  it("should return 'rd' for numbers ending in 3", () => {
    expect(toOrdinal(3)).toBe("3rd");
    expect(toOrdinal(23)).toBe("23rd");
    expect(toOrdinal(33)).toBe("33rd");
    expect(toOrdinal(103)).toBe("103rd");
    expect(toOrdinal(123)).toBe("123rd");
  });

  it("should return 'th' for 11, 12, 13", () => {
    expect(toOrdinal(11)).toBe("11th");
    expect(toOrdinal(12)).toBe("12th");
    expect(toOrdinal(13)).toBe("13th");
  });

  it("should return 'th' for 111, 112, 113", () => {
    expect(toOrdinal(111)).toBe("111th");
    expect(toOrdinal(112)).toBe("112th");
    expect(toOrdinal(113)).toBe("113th");
  });

  it("should return 'th' for numbers ending in 4-9 and 0", () => {
    expect(toOrdinal(4)).toBe("4th");
    expect(toOrdinal(5)).toBe("5th");
    expect(toOrdinal(6)).toBe("6th");
    expect(toOrdinal(7)).toBe("7th");
    expect(toOrdinal(8)).toBe("8th");
    expect(toOrdinal(9)).toBe("9th");
    expect(toOrdinal(10)).toBe("10th");
    expect(toOrdinal(20)).toBe("20th");
    expect(toOrdinal(100)).toBe("100th");
  });

  it("should handle 0", () => {
    expect(toOrdinal(0)).toBe("0th");
  });
});

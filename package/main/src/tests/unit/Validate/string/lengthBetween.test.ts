import { lengthBetween, string } from "@/Validate/string";

describe("lengthBetween", () => {
  it("should accept lengths inside the range including both bounds", () => {
    const validator = string([lengthBetween(2, 4)]);
    expect(validator("ab").validate).toBe(true);
    expect(validator("abc").validate).toBe(true);
    expect(validator("abcd").validate).toBe(true);
  });

  it("should reject lengths outside the range with one message", () => {
    const validator = string([lengthBetween(2, 4, "length must be 2 to 4")]);
    expect(validator("a")).toEqual({
      validate: false,
      message: "length must be 2 to 4",
      type: "string",
    });
    expect(validator("abcde")).toEqual({
      validate: false,
      message: "length must be 2 to 4",
      type: "string",
    });
  });

  it("should return an empty message when no message is provided", () => {
    const validator = string([lengthBetween(2, 4)]);
    expect(validator("").validate).toBe(false);
    expect(validator("").message).toBe("");
  });

  it("should measure length in UTF-16 code units", () => {
    const rule = lengthBetween(2, 2);
    expect(rule.validate("\u{1F600}")).toBe(true);
    expect(rule.validate("\u{3042}")).toBe(false);
  });

  it("should reject every value when min is greater than max", () => {
    const rule = lengthBetween(4, 2);
    expect(rule.validate("abc")).toBe(false);
    expect(rule.type).toBe("string");
  });
});

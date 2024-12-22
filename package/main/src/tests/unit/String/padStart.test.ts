import { padStart } from "@/String/padStart";

describe("padStart", () => {
  it("should pad the string with the given padString to the targetLength", () => {
    expect(padStart("abc", 5, " ")).toBe("  abc");
    expect(padStart("abc", 5, "0")).toBe("00abc");
  });

  it("should not add padding if the string is already equal to or longer than the targetLength", () => {
    expect(padStart("abc", 3, " ")).toBe("abc");
    expect(padStart("abcde", 5, "0")).toBe("abcde");
  });

  it("should repeat the padString if necessary", () => {
    expect(padStart("abc", 7, "xy")).toBe("xyxyabc");
  });

  it("should truncate the padString if the combined length exceeds the targetLength", () => {
    expect(padStart("abc", 6, "12345")).toBe("123abc");
  });

  it("should return an empty string if the targetLength is 0", () => {
    expect(padStart("abc", 0, " ")).toBe("abc");
  });

  it("should throw an error if the padString is empty", () => {
    expect(() => padStart("abc", 5, "")).toThrow("padString cannot be empty");
  });
});

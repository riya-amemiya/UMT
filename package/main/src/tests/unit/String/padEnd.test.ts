import { padEnd } from "@/String/padEnd";

describe("padEnd function", () => {
  test("should add padding to the end of string", () => {
    expect(padEnd("abc", 5, " ")).toBe("abc  ");
    expect(padEnd("hello", 10, "!")).toBe("hello!!!!!");
  });

  test("should not modify if original string is already at or longer than target length", () => {
    expect(padEnd("abc", 3, " ")).toBe("abc");
    expect(padEnd("longstring", 5, "!")).toBe("longstring");
  });

  test("should handle padding string with multiple characters", () => {
    expect(padEnd("abc", 10, "de")).toBe("abcdededed");
  });

  test("should return original string if target length is shorter than original length", () => {
    expect(padEnd("abc", 2, " ")).toBe("abc");
  });

  test("should return original string if padding string is empty", () => {
    expect(padEnd("abc", 5, "")).toBe("abc");
  });
});

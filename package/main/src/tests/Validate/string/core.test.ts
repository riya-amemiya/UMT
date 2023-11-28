import { maxLength, minLength, string } from "@/Validate/string";

describe("string validation", () => {
  it("string", () => {
    const validateString = string([]);
    expect(validateString("").validate).toBeTruthy();
    expect(validateString("abc").validate).toBeTruthy();
    // @ts-expect-error
    expect(validateString(123).validate).toBeFalsy();
  });

  it("string with message", () => {
    const validateStringWithMessage = string([], "Must be string");
    expect(validateStringWithMessage("").message).toEqual("");
    expect(validateStringWithMessage("abc").validate).toBeTruthy();
    // @ts-expect-error
    expect(validateStringWithMessage(123).validate).toBeFalsy();
  });

  it("can specify the maximum and minimum number of characters", () => {
    const validateString = string([minLength(3), maxLength(5)]);
    expect(validateString("ab").validate).toBeFalsy();
    expect(validateString("abc").validate).toBeTruthy();
    expect(validateString("abcde").validate).toBeTruthy();
    expect(validateString("abcdef").validate).toBeFalsy();
    expect(validateString("abcdefg").validate).toBeFalsy();
  });
});

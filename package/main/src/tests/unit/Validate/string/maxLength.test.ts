import { maxLength, string } from "@/Validate/string";

describe("maxLength", () => {
  it("maxLength with message", () => {
    const maxLengthValidatorWithMessage = maxLength(3, "Maximum length is 3");
    const validateStringWithMessage = string([maxLengthValidatorWithMessage]);
    expect(validateStringWithMessage("abcd").message).toEqual(
      "Maximum length is 3",
    );
    expect(validateStringWithMessage("abc").validate).toBeTruthy();
    expect(validateStringWithMessage("ab").validate).toBeTruthy();
  });

  it("maxLength without message", () => {
    const maxLengthValidatorWithoutMessage = maxLength(3);
    const validateStringWithoutMessage = string([
      maxLengthValidatorWithoutMessage,
    ]);
    expect(validateStringWithoutMessage("abc").message).toEqual("");
    expect(validateStringWithoutMessage("abcd").validate).toBeFalsy();
  });
});

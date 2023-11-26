import { string, length_ } from "@/Validate/string";

describe("length", () => {
  it("length with message", () => {
    const lengthValidatorWithMessage = length_(3, "Length must be 3");
    const validateStringWithMessage = string([lengthValidatorWithMessage]);
    expect(validateStringWithMessage("abcd").message).toEqual(
      "Length must be 3",
    );
    expect(validateStringWithMessage("abc").validate).toBeTruthy();
    expect(validateStringWithMessage("ab").validate).toBeFalsy();
  });

  it("length without message", () => {
    const lengthValidatorWithoutMessage = length_(3);
    const validateStringWithoutMessage = string([
      lengthValidatorWithoutMessage,
    ]);
    expect(validateStringWithoutMessage("abc").message).toEqual("");
    expect(validateStringWithoutMessage("abcd").validate).toBeFalsy();
  });
});

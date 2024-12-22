import { minLength, string } from "@/Validate/string";

describe("minLength", () => {
  it("minLength with message", () => {
    const minLengthValidatorWithMessage = minLength(3, "Minimum length is 3");
    const validateStringWithMessage = string([minLengthValidatorWithMessage]);
    expect(validateStringWithMessage("").message).toEqual(
      "Minimum length is 3",
    );
    expect(validateStringWithMessage("ab").validate).toBeFalsy();
    expect(validateStringWithMessage("abc").validate).toBeTruthy();
  });

  it("minLength without message", () => {
    const minLengthValidatorWithoutMessage = minLength(3);
    const validateStringWithoutMessage = string([
      minLengthValidatorWithoutMessage,
    ]);
    expect(validateStringWithoutMessage("abc").message).toEqual("");
    expect(validateStringWithoutMessage("ab").validate).toBeFalsy();
  });
});

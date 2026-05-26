import { exactLength, string } from "@/Validate/string";

describe("exactLength", () => {
  it("exactLength with message", () => {
    const lengthValidatorWithMessage = exactLength(3, "Length must be 3");
    const validateStringWithMessage = string([lengthValidatorWithMessage]);
    expect(validateStringWithMessage("abcd").message).toEqual(
      "Length must be 3",
    );
    expect(validateStringWithMessage("abc").validate).toBeTruthy();
    expect(validateStringWithMessage("ab").validate).toBeFalsy();
  });

  it("exactLength without message", () => {
    const lengthValidatorWithoutMessage = exactLength(3);
    const validateStringWithoutMessage = string([
      lengthValidatorWithoutMessage,
    ]);
    expect(validateStringWithoutMessage("abc").message).toEqual("");
    expect(validateStringWithoutMessage("abcd").validate).toBeFalsy();
  });
});

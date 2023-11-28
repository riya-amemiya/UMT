import { string } from "@/Validate";
import { email } from "@/Validate/string/email";

describe("email", () => {
  it("email with message", () => {
    const emailValidatorWithMessage = email("Invalid email format");
    const validateEmailWithMessage = string([emailValidatorWithMessage]);
    expect(validateEmailWithMessage("user@example.com").message).toEqual("");
    expect(validateEmailWithMessage("user@example.com").validate).toBeTruthy();
    expect(validateEmailWithMessage("userexample.com").message).toEqual(
      "Invalid email format",
    );
    expect(validateEmailWithMessage("userexample.com").validate).toBeFalsy();
  });

  it("email without message", () => {
    const emailValidatorWithoutMessage = email();
    const validateEmailWithoutMessage = string([emailValidatorWithoutMessage]);
    expect(validateEmailWithoutMessage("user@example.com").message).toEqual("");
    expect(
      validateEmailWithoutMessage("user@example.com").validate,
    ).toBeTruthy();
    expect(validateEmailWithoutMessage("userexample.com").validate).toBeFalsy();
  });
});

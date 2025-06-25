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

  it("validates various valid email formats", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const validEmails = [
      "test@example.com",
      "user.name@example.com",
      "user+tag@example.com",
      "user-name@example.com",
      "test123@example.com",
      "a@example.com",
      "very.long.email.address@example.com",
      "user_name@example.com",
      "123@example.com",
      "test.email+tag+sorting@example.com",
    ];

    for (const email of validEmails) {
      expect(validateEmail(email).validate).toBeTruthy();
    }
  });

  it("rejects various invalid email formats", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const invalidEmails = [
      "plainaddress",
      "@example.com",
      "user@",
      "user@example",
      "user.example.com",
      "user@@example.com",
      "user name@example.com",
      "user@example .com",
      "user@example,com",
      "",
      " ",
      "user@example.c",
    ];

    for (const email of invalidEmails) {
      expect(validateEmail(email).validate).toBeFalsy();
    }
  });

  it("documents regex limitations as bugs", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const buggyEmails = [
      "user@example..com",
      ".user@example.com",
      "user.@example.com",
      "user@.example.com",
      "user@example.com.",
    ];

    for (const email of buggyEmails) {
      expect(validateEmail(email).validate).toBeFalsy();
    }
  });

  it("handles edge cases and special characters", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const edgeCases = [
      { email: "user+filter@example.com", valid: true },
      { email: "user_test@example.com", valid: true },
      { email: "123456@example.com", valid: true },
      { email: "user!@example.com", valid: false },
      { email: "user#@example.com", valid: false },
      { email: "user$@example.com", valid: false },
      { email: "user%@example.com", valid: false },
      { email: "user&@example.com", valid: false },
      { email: "user'@example.com", valid: false },
      { email: "user*@example.com", valid: false },
      { email: "user/@example.com", valid: false },
      { email: "user=@example.com", valid: false },
      { email: "user?@example.com", valid: false },
      { email: "user^@example.com", valid: false },
      { email: "user`@example.com", valid: false },
      { email: "user{@example.com", valid: false },
      { email: "user|@example.com", valid: false },
      { email: "user}@example.com", valid: false },
      { email: "user~@example.com", valid: false },
    ];

    for (const { email, valid } of edgeCases) {
      expect(validateEmail(email).validate).toBe(valid);
    }
  });

  it("validates different TLD formats", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const tldEmails = [
      { email: "user@example.com", valid: true },
      { email: "user@example.co", valid: true },
      { email: "user@example.org", valid: true },
      { email: "user@example.net", valid: true },
      { email: "user@example.info", valid: true },
    ];

    for (const { email, valid } of tldEmails) {
      expect(validateEmail(email).validate).toBe(valid);
    }
  });

  it("handles boundary length cases", () => {
    const emailValidator = email();
    const validateEmail = string([emailValidator]);

    const shortLocal = "a@example.com";
    expect(validateEmail(shortLocal).validate).toBeTruthy();

    const longLocal = `${"a".repeat(50)}@example.com`;
    expect(validateEmail(longLocal).validate).toBeTruthy();
  });
});

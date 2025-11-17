import { string } from "@/Validate";
import { validateEmail } from "@/Validate/string/validateEmail";

describe("email", () => {
  it("email with message", () => {
    const emailValidatorWithMessage = validateEmail("Invalid email format");
    const validateEmailWithMessage = string([emailValidatorWithMessage]);
    expect(validateEmailWithMessage("user@example.com").message).toEqual("");
    expect(validateEmailWithMessage("user@example.com").validate).toBeTruthy();
    expect(validateEmailWithMessage("userexample.com").message).toEqual(
      "Invalid email format",
    );
    expect(validateEmailWithMessage("userexample.com").validate).toBeFalsy();
  });

  it("email without message", () => {
    const emailValidatorWithoutMessage = validateEmail();
    const validateEmailWithoutMessage = string([emailValidatorWithoutMessage]);
    expect(validateEmailWithoutMessage("user@example.com").message).toEqual("");
    expect(
      validateEmailWithoutMessage("user@example.com").validate,
    ).toBeTruthy();
    expect(validateEmailWithoutMessage("userexample.com").validate).toBeFalsy();
  });

  it("validates various valid email formats", () => {
    const emailValidator = string([validateEmail()]);

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
      expect(emailValidator(email).validate).toBeTruthy();
    }
  });

  it("rejects various invalid email formats", () => {
    const emailValidator = string([validateEmail()]);

    const invalidEmails = [
      "plainaddress",
      "@example.com",
      "user@",
      "user.example.com",
      "user@@example.com",
      "user name@example.com",
      "user@example .com",
      "user@example,com",
      "",
      " ",
    ];

    for (const email of invalidEmails) {
      expect(emailValidator(email).validate).toBeFalsy();
    }
  });

  it("documents regex limitations as bugs", () => {
    const emailValidator = string([validateEmail()]);

    const buggyEmails = [
      "user@example..com",
      "user@.example.com",
      "user@example.com.",
    ];

    for (const email of buggyEmails) {
      expect(emailValidator(email).validate).toBeFalsy();
    }
  });

  it("handles edge cases and special characters", () => {
    const emailValidator = string([validateEmail()]);

    const edgeCases = [
      { email: "user+filter@example.com", valid: true },
      { email: "user_test@example.com", valid: true },
      { email: "123456@example.com", valid: true },
    ];

    for (const { email, valid } of edgeCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("validates different TLD formats", () => {
    const emailValidator = string([validateEmail()]);

    const tldEmails = [
      { email: "user@example.com", valid: true },
      { email: "user@example.co", valid: true },
      { email: "user@example.org", valid: true },
      { email: "user@example.net", valid: true },
      { email: "user@example.info", valid: true },
    ];

    for (const { email, valid } of tldEmails) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles boundary length cases", () => {
    const emailValidator = string([validateEmail()]);

    const shortLocal = "a@example.com";
    expect(emailValidator(shortLocal).validate).toBeTruthy();

    const longLocal = `${"a".repeat(50)}@example.com`;
    expect(emailValidator(longLocal).validate).toBeTruthy();
  });

  it("RFC 822 obsolete cases (historically valid, now obsolete)", () => {
    const emailValidator = string([
      validateEmail("Invalid email format", { level: "rfc822" }),
    ]);

    const obsoleteCases = [
      {
        email: "easy@example",
        valid: true,
        reason:
          "RFC 822 allowed domains without dots, but RFC 2822 made this obsolete",
      },
    ];

    for (const { email, valid } of obsoleteCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles space-related edge cases (RFC 5322)", () => {
    const emailValidator = string([validateEmail()]);

    const spaceCases = [
      {
        email: "what about spaces@example.com",
        valid: false,
        reason: "Spaces aren't allowed between words",
      },
      {
        email: " maybe-like-this @example.com",
        valid: false,
        reason: "Current implementation doesn't handle spec-allowed spaces",
      },
      {
        email: "fed-up-yet@ example.com ",
        valid: false,
        reason: "Domain part spaces not handled by current regex",
      },
    ];

    for (const { email, valid } of spaceCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles comment syntax (RFC 5322 comments in parentheses)", () => {
    const emailValidator = string([validateEmail()]);

    const commentCases = [
      {
        email: "normal(wtf is this?)@example.com",
        valid: false,
        reason: "Comments technically valid in RFC 822, obsolete in RFC 5322",
      },
      {
        email: "(@)@example.com",
        valid: false,
        reason: "Comments don't count as part of email address",
      },
    ];

    for (const { email, valid } of commentCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles quoted string syntax (RFC 5322)", () => {
    const emailValidator = string([validateEmail()]);

    const quotedCases = [
      {
        email: '":(){|:&};:"@example.com',
        valid: false,
        reason:
          "Quoted strings allow special characters but not supported by current regex",
      },
      {
        email: '""@example.com',
        valid: false,
        reason: "Empty quoted local part technically valid but not supported",
      },
      {
        email: '"@"@[@]',
        valid: false,
        reason: "Complex quoting not supported by current implementation",
      },
      {
        email: "\"'()'\"(\"''\")@example.com",
        valid: false,
        reason: "Complex escaped quotes not supported",
      },
    ];

    for (const { email, valid } of quotedCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles IPv6 address literals (RFC 5321)", () => {
    const emailValidator = string([validateEmail()]);

    const ipv6Cases = [
      {
        email: "magic@[::1]",
        valid: false,
        reason: "IPv6 addresses in brackets not supported by current regex",
      },
      {
        email: "test@[127.0.0.1]",
        valid: false,
        reason: "IP literals not supported",
      },
    ];

    for (const { email, valid } of ipv6Cases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles Unicode and internationalized email addresses (RFC 6532)", () => {
    const emailValidator = string([validateEmail()]);

    const unicodeCases = [
      {
        email: "poop@[👀.👀]",
        valid: false,
        reason: "Unicode in domain brackets per RFC 6532 not supported",
      },
      {
        email: "👉@👈",
        valid: false,
        reason: "Unicode domains not supported by current regex",
      },
      {
        email: "c̷̨̈́i̵̮̅l̶̠̐͊͝ȁ̷̠̗̆̍̍n̷͖̘̯̍̈͒̅t̶͍͂͋ř̵̞͈̓ȯ̷̯̠-̸͚̖̟͋s̴͉̦̭̔̆̃͒û̵̥̪͆̒̕c̸̨̨̧̺̎k̵̼͗̀s̸̖̜͍̲̈́͋̂͠@example.com",
        valid: false,
        reason: "Zalgo text (Unicode combining characters) not supported",
      },
    ];

    for (const { email, valid } of unicodeCases) {
      expect(emailValidator(email).validate).toBe(valid);
    }
  });

  it("handles RFC 5322 length limitations", () => {
    const emailValidator = string([
      validateEmail("Invalid email format", { level: "rfc5322" }),
    ]);

    const longLocalPart = "a".repeat(320);
    const veryLongEmail = `${longLocalPart}@example.com`;

    expect(emailValidator(veryLongEmail).validate).toBe(false);

    const excessivelyLongLocal =
      "according-to-all-known-laws-of-aviation-there-is-no-way-a-bee-should-be-able-to-fly-its-wings-are-too-small-to-get-its-fat-little-body-off-the-ground-the-bee-of-course-flies-anyway-because-bees-don-t-care-what-humans-think-is-impossible-yellow-black-yellow-black-yellow-black-yellow-black-ooh-black-and-yellow-let-s-shake-it-up-a-little-barry-breakfast-is-ready-coming-hang-on-a-second-hello-barry-adam-can-you-believe-this-is-happening-i-can-t-i-ll-pick-you-up-looking-sharp-use-the-stairs-your-father-paid-good-money-for-those-sorry-i-m-excited-here-s-the-graduate-we-re-very-proud-of-you-son-a-perfect-report-card-all-b-s-very-proud-ma-i-got-a-thing-going-here-you-got-lint-on-your-fuzz-ow-that-s-me-wave-to-us-we-ll-be-in-row-118-000-bye-barry-i-told-you-stop-flying-in-the-house-hey-adam-hey-barry-is-that-fuzz-gel-a-little-special-day-graduation-never-thought-i-d-make-it-three-days-grade-school-three-days-high-school-those-were-awkward-three-days-college-i-m-glad-i-took-a-day-and-hitchhiked-around-the-hive-you-did-come-back-different-hi-barry-artie-growing-a-mustache-looks-good-hear-about-frankie-yeah-you-going-to-the-funeral-no-i-m-not-going-everybody-knows-sting-someone-you-die-don-t-waste-it-on-a-squirrel-such-a-hothead-i-guess-he-could-have-just-gotten-out-of-the-way-i-love-this-incorporating-an-amusement-park-into-our-day-that-s-why-we-don-t-need-vacations-boy-quite-a-bit-of-pomp-under-the-circumstances-well-adam-today-we-are-men-we-are-bee-men-amen-hallelujah-students-faculty-distinguished-bees-please-welcome-dean-buzzwell-welcome-new-hive-city-graduating-class-of-9-15-that-concludes-our-ceremonies-and-begins-your-career-at-honex-industries-will-we-pick-our-job-today-i-heard-it-s-just-orientation-heads-up-here-we-go-keep-your-hands-and-antennas-inside-the-tram-at-all-times-wonder-what-it-ll-be-like-a-little-scary-welcome-to-honex-a-division-of-honesco-and-a-part-of-the-hexagon-group-this-is-it-wow-wow-we-know-that-you-as-a-bee-have-worked-your-whole-life-to-get-to-the-point-where-you-can-work-for-your-whole-life-honey-begins-when-our-valiant-pollen-jocks-bring-the-nectar-to-the-hive-our-top-secret-formula-is-automatically-color-corrected-scent-adjusted-and-bubble-contoured-into-this-soothing-sweet-syrup-with-its-distinctive-golden-glow-you-know-as-honey-that-girl-was-hot-she-s-my-cousin-she-is-yes-we-re-all-cousins-right-you-re-right-at-honex-we-constantly-strive-to-improve-every-aspect-of-bee-existence-these-bees-are-stress-testing-a-new-helmet-technology-what-do-you-think-he-makes-not-enough-here-we-have-our-latest-advancement-the-krelman-what-does-that-do-catches-that-little-strand-of-honey-that-hangs-after-you-pour-it-saves-us-millions-can-anyone-work-on-the-krelman-of-course-most-bee-jobs-are-small-ones@example.com";

    expect(emailValidator(excessivelyLongLocal).validate).toBe(false);
  });
});

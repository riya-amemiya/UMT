import { parseEmail, type ParseEmailLevel } from "@/Validate/parseEmail";

describe("email", () => {
  it("email with message", () => {
    expect(
      parseEmail({ email: "user@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "user@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "userexample.com", options: { level: "basic" } })
        .valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: "userexample.com", options: { level: "basic" } })
        .valid,
    ).toBeFalsy();
  });

  it("email without message", () => {
    expect(
      parseEmail({ email: "user@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "user@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "userexample.com", options: { level: "basic" } })
        .valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: "userexample.com", options: { level: "basic" } })
        .valid,
    ).toBeFalsy();
  });

  it("validates various valid email formats", () => {
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
      expect(
        parseEmail({ email, options: { level: "basic" } }).valid,
      ).toBeTruthy();
    }
  });

  it("rejects various invalid email formats", () => {
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
      expect(
        parseEmail({ email, options: { level: "basic" } }).valid,
      ).toBeFalsy();
    }
  });

  it("validates different TLD formats", () => {
    const tldEmails = [
      { email: "user@example.com", valid: true },
      { email: "user@example.co", valid: true },
      { email: "user@example.org", valid: true },
      { email: "user@example.net", valid: true },
      { email: "user@example.info", valid: true },
    ];

    for (const { email, valid } of tldEmails) {
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("handles boundary length cases", () => {
    const shortLocal = "a@example.com";
    expect(
      parseEmail({ email: shortLocal, options: { level: "basic" } }).valid,
    ).toBeTruthy();

    const longLocal = `${"a".repeat(50)}@example.com`;
    expect(
      parseEmail({ email: longLocal, options: { level: "basic" } }).valid,
    ).toBeTruthy();
  });

  it("handles space-related edge cases (RFC 5322)", () => {
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
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("handles comment syntax (RFC 5322 comments in parentheses)", () => {
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
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("handles quoted string syntax (RFC 5322)", () => {
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
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("handles IPv6 address literals (RFC 5321)", () => {
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
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("handles Unicode and internationalized email addresses (RFC 6532)", () => {
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
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        valid,
      );
    }
  });

  it("validates with different RFC compliance levels", () => {
    expect(
      parseEmail({ email: "test@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "test@example.com", options: { level: "rfc822" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "test@example.com",
        options: { level: "rfc2822" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "test@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "test@example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC 822 allows domains without TLD", () => {
    expect(
      parseEmail({ email: "user@localhost", options: { level: "rfc822" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "user@example", options: { level: "rfc822" } }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: "user@localhost", options: { level: "rfc5321" } })
        .valid,
    ).toBeFalsy();
  });

  it("RFC 822 accepts special characters in local part", () => {
    expect(
      parseEmail({
        email: "user!test@localhost",
        options: { level: "rfc822" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user#test@localhost",
        options: { level: "rfc822" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user$test@localhost",
        options: { level: "rfc822" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC 2822 requires TLD and applies length checks", () => {
    expect(
      parseEmail({ email: "user@localhost", options: { level: "rfc2822" } })
        .valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user@example.com",
        options: { level: "rfc2822" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user!test@example.com",
        options: { level: "rfc2822" },
      }).valid,
    ).toBeTruthy();
    const veryLongEmail = `${"a".repeat(1000)}@example.com`;
    expect(
      parseEmail({ email: veryLongEmail, options: { level: "rfc2822" } }).valid,
    ).toBeFalsy();
  });

  it("RFC 5321 allows RFC 5322 special characters in local-part but not in domain", () => {
    expect(
      parseEmail({
        email: "user@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user+tag@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user.name@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user!test@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user#test@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user@exam!ple.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user@exam#ple.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: "user@localhost", options: { level: "rfc5321" } })
        .valid,
    ).toBeFalsy();
  });

  it("RFC 5322 accepts all special characters from RFC", () => {
    const rfc5322ValidChars = "!#$%&'*+/=?^_`{|}~-";
    for (const char of rfc5322ValidChars) {
      const email = `user${char}test@example.com`;
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("basic level only accepts common characters", () => {
    expect(
      parseEmail({ email: "user@example.com", options: { level: "basic" } })
        .valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user.name@example.com",
        options: { level: "basic" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user_name@example.com",
        options: { level: "basic" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user+tag@example.com",
        options: { level: "basic" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user-name@example.com",
        options: { level: "basic" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC levels reject consecutive dots", () => {
    expect(
      parseEmail({
        email: "user..name@example.com",
        options: { level: "rfc2822" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user..name@example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user..name@example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeFalsy();
  });

  it("RFC levels reject leading/trailing dots in local part", () => {
    expect(
      parseEmail({
        email: ".user@example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user.@example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeFalsy();
  });

  it("RFC levels respect local part length limit of 64 chars", () => {
    const longLocal = `${"a".repeat(65)}@example.com`;
    expect(
      parseEmail({ email: longLocal, options: { level: "rfc822" } }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: longLocal, options: { level: "rfc5322" } }).valid,
    ).toBeFalsy();

    const exactLimit = `${"a".repeat(64)}@example.com`;
    expect(
      parseEmail({ email: exactLimit, options: { level: "rfc822" } }).valid,
    ).toBeTruthy();
  });

  it("RFC levels with length checks enforce total limit", () => {
    const veryLongEmail = `${"a".repeat(990)}@example.com`;
    expect(
      parseEmail({ email: veryLongEmail, options: { level: "rfc2822" } }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: veryLongEmail, options: { level: "rfc5321" } }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: veryLongEmail, options: { level: "rfc5322" } }).valid,
    ).toBeFalsy();
  });

  it("RFC 822 vs others: TLD requirement difference", () => {
    const noTldEmail = "user@host";
    expect(
      parseEmail({ email: noTldEmail, options: { level: "rfc822" } }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: noTldEmail, options: { level: "rfc2822" } }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: noTldEmail, options: { level: "rfc5321" } }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({ email: noTldEmail, options: { level: "rfc5322" } }).valid,
    ).toBeFalsy();
  });

  it("extracts email parts correctly", () => {
    const result1 = parseEmail({
      email: "user@example.com",
      options: { level: "basic" },
    });
    expect(result1.valid).toBeTruthy();
    expect(result1.parts?.local).toBe("user");
    expect(result1.parts?.domain).toBe("example.com");

    const result2 = parseEmail({
      email: "test.user@sub.example.com",
      options: { level: "basic" },
    });
    expect(result2.valid).toBeTruthy();
    expect(result2.parts?.local).toBe("test.user");
    expect(result2.parts?.domain).toBe("sub.example.com");

    const result3 = parseEmail({
      email: "invalid",
      options: { level: "basic" },
    });
    expect(result3.valid).toBeFalsy();
    expect(result3.parts).toBeUndefined();
  });

  it("validates domain name rules more strictly (RFC 1035/5321)", () => {
    expect(
      parseEmail({
        email: "user@-example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user@example-.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user@example.com-",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();

    const longLabel = "a".repeat(64);
    expect(
      parseEmail({
        email: `user@${longLabel}.com`,
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();

    const longDomain = `user@example.${"a".repeat(250)}.com`;
    expect(
      parseEmail({ email: longDomain, options: { level: "basic" } }).valid,
    ).toBeFalsy();

    const exactLabel = "a".repeat(63);
    expect(
      parseEmail({
        email: `user@${exactLabel}.com`,
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();

    expect(
      parseEmail({
        email: "user@123.example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
  });

  it("handles quoted strings correctly based on RFC 5322 level", () => {
    const quotedEmails = [
      '"user with space"@example.com',
      '"user..dots"@example.com',
      '".leadingdot"@example.com',
      '"trailingdot."@example.com',
      '"user\\"quote"@example.com',
      '"user\\\\backslashes"@example.com',
    ];

    for (const email of quotedEmails) {
      expect(
        parseEmail({ email, options: { level: "basic" } }).valid,
      ).toBeFalsy();
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("handles domain literals (IP addresses) correctly based on RFC 5321/5322 level", () => {
    const literalEmails = ["user@[192.168.0.1]", "user@[IPv6:2001:db8::1]"];

    for (const email of literalEmails) {
      expect(
        parseEmail({ email, options: { level: "basic" } }).valid,
      ).toBeFalsy();
      expect(
        parseEmail({ email, options: { level: "rfc5321" } }).valid,
      ).toBeTruthy();
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }

    expect(
      parseEmail({
        email: "user@[127.0.0.1.1]",
        options: { level: "basic" },
      }).valid,
    ).toBeFalsy();
    expect(
      parseEmail({
        email: "user@[IPv6:not-an-ip]",
        options: { level: "basic" },
      }).valid,
    ).toBeFalsy();
  });

  it("handles comments correctly based on RFC 5322 level", () => {
    const commentEmails = [
      "user(comment)@example.com",
      "user@(comment)example.com",
      "(comment)user@example.com",
    ];

    for (const email of commentEmails) {
      expect(
        parseEmail({ email, options: { level: "basic" } }).valid,
      ).toBeFalsy();
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }

    const parts = parseEmail({
      email: "user(comment)@example.com",
      options: { level: "rfc5322" },
    });
    expect(parts.valid).toBeTruthy();

    const parts2 = parseEmail({
      email: "user@(comment)example.com",
      options: { level: "rfc5322" },
    });
    expect(parts2.valid).toBeTruthy();
    expect(parts2.parts?.domain).toBe("example.com");
  });

  it("handles Punycode (IDN) addresses", () => {
    const punycodeEmail = "user@xn--bcher-kva.com";
    expect(
      parseEmail({ email: punycodeEmail, options: { level: "basic" } }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({ email: punycodeEmail, options: { level: "rfc5321" } }).valid,
    ).toBeTruthy();
  });

  it("correctly normalizes domain case in parts extraction", () => {
    const result = parseEmail({
      email: "user@EXAMPLE.COM",
      options: { level: "basic" },
    });
    expect(result.valid).toBeTruthy();
    expect(result.parts?.domain).toBe("EXAMPLE.COM");

    const result2 = parseEmail({
      email: "user+name@SUB.EXAMPLE.CO.UK",
      options: { level: "basic" },
    });
    expect(result2.valid).toBeTruthy();
    expect(result2.parts?.domain).toBe("SUB.EXAMPLE.CO.UK");
  });

  it("RFC 5322: handles quoted strings with special characters", () => {
    const emails = [
      '"@+;,"@example.com',
      '"test@test"@example.com',
      '"a\\ b"@example.com',
    ];
    for (const email of emails) {
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("RFC 5322: empty quoted string in local-part is valid", () => {
    expect(
      parseEmail({ email: '""@example.com', options: { level: "rfc5322" } })
        .valid,
    ).toBeTruthy();
  });

  it("RFC 5322: handles various comment placements", () => {
    const emails = [
      "(comment)user@example.com",
      "user(comment)@example.com",
      "user@(comment)example.com",
      "user@example.com(comment)",
      "user@(comment)[127.0.0.1]",
    ];
    for (const email of emails) {
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("RFC 5322: handles nested comments and comments around dots", () => {
    const emails = [
      "user(comment(nested))@example.com",
      "first.(comment)last@example.com",
      "user@example.(comment)com",
    ];
    for (const email of emails) {
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("RFC 5322: handles comments with quoted characters", () => {
    expect(
      parseEmail({
        email: "user(a\\(b\\)c)@example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC 5321/5322: handles domain literals correctly", () => {
    expect(
      parseEmail({
        email: "user@[192.168.1.1]",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user@[IPv6:2001:db8::1]",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user@[192.168.1.1]",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
    expect(
      parseEmail({
        email: "user@[IPv6:2001:db8::1]",
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC 5321: path length (max 256 octets)", () => {
    const d63 = "a".repeat(63);
    const domain249 = `${d63}.${d63}.${d63}.${"a".repeat(57)}`;
    expect(domain249.length).toBe(249);

    // 5 + 1 + 249 = 255 <= 256 -> valid
    const local5 = "a".repeat(5);
    expect(
      parseEmail({
        email: `${local5}@${domain249}`,
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();

    // 6 + 1 + 249 = 256 <= 256 -> valid
    const local6 = "a".repeat(6);
    expect(
      parseEmail({
        email: `${local6}@${domain249}`,
        options: { level: "rfc5321" },
      }).valid,
    ).toBeTruthy();

    // 7 + 1 + 249 = 257 > 256 -> invalid
    const local7 = "a".repeat(7);
    expect(
      parseEmail({
        email: `${local7}@${domain249}`,
        options: { level: "rfc5321" },
      }).valid,
    ).toBeFalsy();
  });

  it("RFC 5322 obsolete: handles mixed quoted and unquoted local-parts", () => {
    const emails = ['"first".last@example.com', 'first."last"@example.com'];
    for (const email of emails) {
      expect(
        parseEmail({ email, options: { level: "rfc5322" } }).valid,
      ).toBeTruthy();
    }
  });

  it("RFC 5322: handles extra whitespace around @", () => {
    expect(
      parseEmail({
        email: "user  @  example.com",
        options: { level: "rfc5322" },
      }).valid,
    ).toBeTruthy();
  });

  it("RFC 822: handles specific quoted pairs", () => {
    const emails = [
      '"first\\ last"@host',
      '"\\"quote\\""@host',
      '"back\\\\slash"@host',
    ];
    for (const email of emails) {
      expect(
        parseEmail({ email, options: { level: "rfc822" } }).valid,
      ).toBeTruthy();
    }
  });

  it("RFC 5322: should handle comments within the domain part", () => {
    const commentInDomain = {
      email: "user@example.(comment)com",
      valid: true,
      reason: "Comments are allowed within the domain part in RFC 5322.",
    };
    expect(
      parseEmail({
        email: commentInDomain.email,
        options: { level: "rfc5322" },
      }).valid,
    ).toBe(commentInDomain.valid);
    expect(
      parseEmail({
        email: commentInDomain.email,
        options: { level: "basic" },
      }).valid,
    ).toBe(false);
  });

  it("RFC 5322: should handle whitespace around the '@' symbol", () => {
    const whitespaceAroundAt = [
      { email: "user @example.com", valid: true },
      { email: "user@ example.com", valid: true },
      { email: "user @ example.com", valid: true },
    ];
    for (const { email, valid } of whitespaceAroundAt) {
      expect(parseEmail({ email, options: { level: "rfc5322" } }).valid).toBe(
        valid,
      );
      expect(parseEmail({ email, options: { level: "basic" } }).valid).toBe(
        false,
      );
    }
  });

  it("RFC 822: should reject obsolete source route syntax in addr-spec", () => {
    const obsoleteRoute = {
      email: "@route.com:user@example.com",
      valid: false,
      reason:
        "Source routes are not part of the addr-spec and should be rejected.",
    };
    expect(
      parseEmail({
        email: obsoleteRoute.email,
        options: { level: "rfc822" },
      }).valid,
    ).toBe(obsoleteRoute.valid);
    expect(
      parseEmail({ email: obsoleteRoute.email, options: { level: "basic" } })
        .valid,
    ).toBe(obsoleteRoute.valid);
  });

  it("RFC 1035: should reject domain labels starting or ending with a hyphen", () => {
    expect(
      parseEmail({
        email: "user@-example.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBe(false);
    expect(
      parseEmail({
        email: "user@example-.com",
        options: { level: "rfc5321" },
      }).valid,
    ).toBe(false);
  });

  it("rejects completely invalid strings across all levels", () => {
    const completelyInvalidStrings = [
      "not-an-email-at-all",
      "12345",
      "!@#$%^&*()",
      "random string with spaces",
      "just-text",
      "<script>alert('xss')</script>",
      "../../../etc/passwd",
      "SELECT * FROM users",
      "null",
      "undefined",
      "true",
      "false",
      "{}",
      "[]",
      "0",
      "NaN",
    ];

    const levels: ParseEmailLevel[] = [
      "basic",
      "rfc822",
      "rfc2822",
      "rfc5321",
      "rfc5322",
    ];

    for (const str of completelyInvalidStrings) {
      for (const level of levels) {
        expect(parseEmail({ email: str, options: { level } }).valid).toBe(
          false,
        );
      }
    }
  });

  it("RFC levels reject email-like but invalid strings", () => {
    const emailLikeButInvalid = [
      "no-at-sign.example.com",
      "multiple@@at@signs.example.com",
      "@no-local-part.example.com",
      "no-domain@",
      "..consecutive@example.com",
      "trailing..dots@example.com",
      "local@.leading-domain.example.com",
      "local@trailing-dot.example.com.",
      "local@domain..consecutive.example.com",
      "user@@example.com",
      "@@@example.com",
      "user@@@",
      "user@",
      "@domain.example.com",
    ];

    const levels: ParseEmailLevel[] = [
      "rfc822",
      "rfc2822",
      "rfc5321",
      "rfc5322",
    ];

    for (const email of emailLikeButInvalid) {
      for (const level of levels) {
        expect(parseEmail({ email, options: { level } }).valid).toBe(false);
      }
    }
  });

  it("rejects strings with invalid control characters on stricter levels", () => {
    const invalidControlChars = [
      "user\n@example.com",
      "user@example.com\n",
      "\nuser@example.com",
      "user@exam\nple.com",
      "user\r@example.com",
      "user@example.com\r",
      "\ruser@example.com",
      "user@exam\rple.com",
    ];

    const levels: ParseEmailLevel[] = [
      "basic",
      "rfc2822",
      "rfc5321",
      "rfc5322",
    ];

    for (const email of invalidControlChars) {
      for (const level of levels) {
        expect(parseEmail({ email, options: { level } }).valid).toBe(false);
      }
    }
  });

  it("RFC levels reject emails with excessive length violations", () => {
    const tooLongLocal = `${"a".repeat(65)}@example.com`;
    const tooLongTotal = `${"a".repeat(990)}@example.com`;

    const levelsWithLocalLengthCheck: ParseEmailLevel[] = [
      "rfc2822",
      "rfc5322",
    ];

    for (const level of levelsWithLocalLengthCheck) {
      expect(
        parseEmail({ email: tooLongLocal, options: { level } }).valid,
      ).toBe(false);
    }

    const levelsWithTotalLengthCheck: ParseEmailLevel[] = [
      "rfc2822",
      "rfc5321",
      "rfc5322",
    ];

    for (const level of levelsWithTotalLengthCheck) {
      expect(
        parseEmail({ email: tooLongTotal, options: { level } }).valid,
      ).toBe(false);
    }
  });

  it("rejects malformed domain structures across all levels", () => {
    const malformedDomains = [
      "user@",
      "user@.",
      "user@..",
      "user@...",
      "user@.domain.example.com",
      "user@domain..example.com",
    ];

    const levels: ParseEmailLevel[] = [
      "basic",
      "rfc822",
      "rfc2822",
      "rfc5321",
      "rfc5322",
    ];

    for (const email of malformedDomains) {
      for (const level of levels) {
        expect(parseEmail({ email, options: { level } }).valid).toBe(false);
      }
    }
  });

  it("rejects invalid characters and patterns that should fail on basic and stricter levels", () => {
    const basicInvalidEmails = [
      "user name@example.com",
      "user\t@example.com",
      "user@exam ple.com",
      "user@exam\tple.com",
      "spaces in local@example.com",
      "local@spaces in domain.example.com",
    ];

    const basicLevels: ParseEmailLevel[] = ["basic"];

    for (const email of basicInvalidEmails) {
      for (const level of basicLevels) {
        expect(parseEmail({ email, options: { level } }).valid).toBe(false);
      }
    }
  });
});

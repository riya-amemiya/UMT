package validate_test

import (
	"regexp"
	"testing"

	umt_validate "github.com/riya-amemiya/umt-go/src/validate"
)

// --- ExactLength (mirrors Validate/string/exactLength.test.ts) ---

func TestExactLength_WithMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.ExactLength(3, "Length must be 3")},
		"",
	)
	if got := v("abcd").Message; got != "Length must be 3" {
		t.Fatalf("expected custom message, got %q", got)
	}
	if !v("abc").Validate {
		t.Fatalf("expected length 3 valid")
	}
	if v("ab").Validate {
		t.Fatalf("expected length 2 invalid")
	}
}

func TestExactLength_WithoutMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.ExactLength(3, "")},
		"",
	)
	if got := v("abc").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if v("abcd").Validate {
		t.Fatalf("expected length 4 invalid")
	}
}

// --- MinLength (mirrors Validate/string/minLength.test.ts) ---

func TestMinLength_WithMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.MinLength(3, "Minimum length is 3")},
		"",
	)
	if got := v("").Message; got != "Minimum length is 3" {
		t.Fatalf("expected custom message, got %q", got)
	}
	if v("ab").Validate {
		t.Fatalf("expected length 2 invalid")
	}
	if !v("abc").Validate {
		t.Fatalf("expected length 3 valid")
	}
}

func TestMinLength_WithoutMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.MinLength(3, "")},
		"",
	)
	if got := v("abc").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if v("ab").Validate {
		t.Fatalf("expected length 2 invalid")
	}
}

// --- MaxLength (mirrors Validate/string/maxLength.test.ts) ---

func TestMaxLength_WithMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.MaxLength(3, "Maximum length is 3")},
		"",
	)
	if got := v("abcd").Message; got != "Maximum length is 3" {
		t.Fatalf("expected custom message, got %q", got)
	}
	if !v("abc").Validate {
		t.Fatalf("expected length 3 valid")
	}
	if !v("ab").Validate {
		t.Fatalf("expected length 2 valid")
	}
}

func TestMaxLength_WithoutMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.MaxLength(3, "")},
		"",
	)
	if got := v("abc").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if v("abcd").Validate {
		t.Fatalf("expected length 4 invalid")
	}
}

// --- NumberString (mirrors Validate/string/numberString.test.ts) ---

func TestNumberString_ValidStrings(t *testing.T) {
	rule := umt_validate.NumberString("")
	if !rule.Validate("123") {
		t.Fatalf("expected \"123\" valid")
	}
	if !rule.Validate("0.56") {
		t.Fatalf("expected \"0.56\" valid")
	}
}

func TestNumberString_InvalidStrings(t *testing.T) {
	rule := umt_validate.NumberString("")
	if rule.Validate("abc") {
		t.Fatalf("expected \"abc\" invalid")
	}
	if rule.Validate("123abc") {
		t.Fatalf("expected \"123abc\" invalid")
	}
	if !rule.Validate("") {
		t.Fatalf("expected empty string valid")
	}
}

func TestNumberString_CustomMessageSurfacedThroughString(t *testing.T) {
	customMessage := "This is not a valid number string"
	rule := umt_validate.NumberString(customMessage)
	if rule.Validate("abc") {
		t.Fatalf("expected \"abc\" invalid")
	}
	v := umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{rule}, "")
	if got := v("abc").Message; got != customMessage {
		t.Fatalf("expected custom message, got %q", got)
	}
}

// --- OneOf (mirrors Validate/string/oneOf.test.ts) ---

func TestOneOf_ValidatesIncludedValue(t *testing.T) {
	v := umt_validate.OneOf([]string{"standard", "squat", "decanter", "round", "tall", "flask"}, "")
	if !v("standard").Validate {
		t.Fatalf("expected standard accepted")
	}
	if !v("flask").Validate {
		t.Fatalf("expected flask accepted")
	}
	if v("unknown").Validate {
		t.Fatalf("expected unknown rejected")
	}
}

func TestOneOf_CustomMessageOnFailure(t *testing.T) {
	v := umt_validate.OneOf([]string{"a", "b"}, "must be a or b")
	if got := v("c").Message; got != "must be a or b" {
		t.Fatalf("expected custom message, got %q", got)
	}
	if !v("a").Validate {
		t.Fatalf("expected a accepted")
	}
	if got := v("a").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
}

func TestOneOf_EmptyMessageWhenNoneProvided(t *testing.T) {
	v := umt_validate.OneOf([]string{"x", "y"}, "")
	if got := v("z").Message; got != "" {
		t.Fatalf("expected empty message, got %q", got)
	}
	if v("z").Validate {
		t.Fatalf("expected z rejected")
	}
}

func TestOneOf_RejectsEmptyAllowedList(t *testing.T) {
	v := umt_validate.OneOf([]string{}, "")
	if v("anything").Validate {
		t.Fatalf("expected empty allowed list to reject everything")
	}
}

func TestOneOf_CaseSensitive(t *testing.T) {
	v := umt_validate.OneOf([]string{"Tall"}, "")
	if !v("Tall").Validate {
		t.Fatalf("expected Tall accepted")
	}
	if v("tall").Validate {
		t.Fatalf("expected tall rejected (case sensitive)")
	}
}

func TestOneOf_ExposesValueThroughType(t *testing.T) {
	v := umt_validate.OneOf([]string{"a", "b"}, "")
	if got := v("a").Type; got != "a" {
		t.Fatalf("expected type a, got %q", got)
	}
	if got := v("b").Type; got != "b" {
		t.Fatalf("expected type b, got %q", got)
	}
}

// --- RegexMatch (mirrors Validate/string/regexMatch.test.ts) ---

func TestRegexMatch_MatchingPattern(t *testing.T) {
	rule := umt_validate.RegexMatch(regexp.MustCompile(`^[a-z]+$`), "")
	if !rule.Validate("abc") {
		t.Fatalf("expected abc to match")
	}
	if rule.Validate("ABC") {
		t.Fatalf("expected ABC not to match")
	}
	if rule.Validate("123") {
		t.Fatalf("expected 123 not to match")
	}
	if rule.Validate("abc123") {
		t.Fatalf("expected abc123 not to match")
	}
}

func TestRegexMatch_NonMatchingPattern(t *testing.T) {
	rule := umt_validate.RegexMatch(regexp.MustCompile(`^[0-9]+$`), "")
	if !rule.Validate("123") {
		t.Fatalf("expected 123 to match")
	}
	if rule.Validate("abc") {
		t.Fatalf("expected abc not to match")
	}
	if rule.Validate("123abc") {
		t.Fatalf("expected 123abc not to match")
	}
	if rule.Validate("") {
		t.Fatalf("expected empty string not to match")
	}
}

func TestRegexMatch_CustomMessageSurfacedThroughString(t *testing.T) {
	message := "Only uppercase letters are allowed"
	rule := umt_validate.RegexMatch(regexp.MustCompile(`^[A-Z]+$`), message)
	if !rule.Validate("ABC") {
		t.Fatalf("expected ABC to match")
	}
	if rule.Validate("abc") {
		t.Fatalf("expected abc not to match")
	}
	v := umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{rule}, "")
	if got := v("abc").Message; got != message {
		t.Fatalf("expected custom message, got %q", got)
	}
}

// --- Uuid (mirrors Validate/string/uuid.test.ts) ---

func TestUuid_ValidUUIDs(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	valid := []string{
		"123e4567-e89b-42d3-a456-426614174000",
		"3A86B1AB-237E-473D-A38F-06AA6A2A4783",
		"B86D03E7-7B2D-4710-897B-77D05A1F0B4B",
		"573F42D5-1734-45E1-9D4D-AA43E6DF697E",
		"D1643B1F-DCEC-46A0-9196-D2C9B8446601",
		"44046745-283C-4507-955E-430DAAD16189",
		"7185AF69-0153-4C42-B63F-368291A74AAF",
		"ADD5ABC1-54B8-4D5D-A8A0-ABA55089A1CF",
		"0C338BD0-768B-4510-95D0-0D1571B08405",
		"C48EACCC-CEA3-4C4D-B669-0D1669D4DBBF",
		"A95E3F62-2D0E-4257-95D3-B239AF08115D",
		"A95E3F622D0E425795D3B239AF08115D",
	}
	for _, value := range valid {
		if !rule.Validate(value) {
			t.Fatalf("expected %q to be a valid v4 UUID", value)
		}
	}
}

func TestUuid_RejectsInvalidUUIDs(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if rule.Validate("123e4567-e89b-42d3-a456-42661417400Z") {
		t.Fatalf("expected non-hex character to be rejected")
	}
	if rule.Validate("123e4567-e89b-92d3-a456-426614174000") {
		t.Fatalf("expected wrong version digit to be rejected")
	}
}

func TestUuid_ValidWithCustomMessage(t *testing.T) {
	validUUID := "123e4567-e89b-42d3-a456-426614174000"
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.Uuid([]int{4}, "Invalid UUID format")},
		"",
	)
	if !v(validUUID).Validate {
		t.Fatalf("expected valid UUID accepted")
	}
	if got := v(validUUID).Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
}

func TestUuid_InvalidWithCustomMessage(t *testing.T) {
	invalidUUID := "123e4567-e89b-12d3-a456-42661417400Z"
	customMessage := "Invalid UUID format"
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.Uuid([]int{4}, customMessage)},
		"",
	)
	if v(invalidUUID).Validate {
		t.Fatalf("expected invalid UUID rejected")
	}
	if got := v(invalidUUID).Message; got != customMessage {
		t.Fatalf("expected custom message, got %q", got)
	}
}

func TestUuid_MixedCase(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if !rule.Validate("123e4567-E89b-42D3-a456-426614174000") {
		t.Fatalf("expected mixed-case UUID accepted")
	}
}

func TestUuid_DifferentVersions(t *testing.T) {
	rule := umt_validate.Uuid([]int{1, 2, 3, 4, 5}, "")
	values := []string{
		"123e4567-e89b-12d3-a456-426614174000",
		"123e4567-e89b-22d3-a456-426614174000",
		"123e4567-e89b-32d3-a456-426614174000",
		"123e4567-e89b-52d3-a456-426614174000",
	}
	for _, value := range values {
		if !rule.Validate(value) {
			t.Fatalf("expected %q accepted for versions 1-5", value)
		}
	}
}

func TestUuid_RejectsCompletelyInvalid(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if rule.Validate("completely-invalid-format") {
		t.Fatalf("expected completely invalid string rejected")
	}
}

func TestUuid_RejectsEmptyString(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if rule.Validate("") {
		t.Fatalf("expected empty string rejected")
	}
}

func TestUuid_RejectsInsufficientLength(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if rule.Validate("123e4567-e89b-42d3-a456-42661417400") {
		t.Fatalf("expected too-short UUID rejected")
	}
}

func TestUuid_RejectsExcessiveLength(t *testing.T) {
	rule := umt_validate.Uuid(nil, "")
	if rule.Validate("123e4567-e89b-42d3-a456-4266141740001234") {
		t.Fatalf("expected too-long UUID rejected")
	}
}

func TestUuid_AllVersionsOutOfRange(t *testing.T) {
	rule := umt_validate.Uuid([]int{0, 8, 99}, "")
	if rule.Validate("123e4567-e89b-42d3-a456-426614174000") {
		t.Fatalf("expected rejection when all versions out of range (v4)")
	}
	if rule.Validate("123e4567-e89b-82d3-a456-426614174000") {
		t.Fatalf("expected rejection when all versions out of range (v8)")
	}
}

func TestUuid_FiltersInvalidVersionsKeepsValid(t *testing.T) {
	rule := umt_validate.Uuid([]int{4, 99}, "")
	if !rule.Validate("123e4567-e89b-42d3-a456-426614174000") {
		t.Fatalf("expected v4 UUID accepted when 99 is filtered out")
	}
}

func TestUuid_EmptyVersionsRejectsEverything(t *testing.T) {
	rule := umt_validate.Uuid([]int{}, "")
	if rule.Validate("123e4567-e89b-42d3-a456-426614174000") {
		t.Fatalf("expected empty versions slice to reject everything")
	}
}

// --- ValidateEmailRule (mirrors Validate/string/email.test.ts) ---

func TestValidateEmailRule_WithMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.ValidateEmailRule("Invalid email format", "")},
		"",
	)
	if got := v("user@example.com").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if !v("user@example.com").Validate {
		t.Fatalf("expected valid email accepted")
	}
	if got := v("userexample.com").Message; got != "Invalid email format" {
		t.Fatalf("expected custom message, got %q", got)
	}
	if v("userexample.com").Validate {
		t.Fatalf("expected invalid email rejected")
	}
}

func TestValidateEmailRule_WithoutMessage(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.ValidateEmailRule("", "")},
		"",
	)
	if got := v("user@example.com").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if !v("user@example.com").Validate {
		t.Fatalf("expected valid email accepted")
	}
	if v("userexample.com").Validate {
		t.Fatalf("expected invalid email rejected")
	}
}

func TestValidateEmailRule_ValidFormats(t *testing.T) {
	rule := umt_validate.ValidateEmailRule("", "")
	valid := []string{
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
	}
	for _, email := range valid {
		if !rule.Validate(email) {
			t.Fatalf("expected %q to be a valid email", email)
		}
	}
}

func TestValidateEmailRule_InvalidFormats(t *testing.T) {
	rule := umt_validate.ValidateEmailRule("", "")
	invalid := []string{
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
		"user@example..com",
		"user@.example.com",
		"user@example.com.",
	}
	for _, email := range invalid {
		if rule.Validate(email) {
			t.Fatalf("expected %q to be rejected", email)
		}
	}
}

func TestValidateEmailRule_RFC822ObsoleteDomainWithoutDot(t *testing.T) {
	v := umt_validate.StringValidator(
		[]umt_validate.ValidateReturnType[string]{umt_validate.ValidateEmailRule("Invalid email format", "rfc822")},
		"",
	)
	if !v("easy@example").Validate {
		t.Fatalf("expected RFC 822 to accept domain without a dot")
	}
}

func TestValidateEmailRule_RFC5322LengthLimit(t *testing.T) {
	rule := umt_validate.ValidateEmailRule("Invalid email format", "rfc5322")
	longLocalPart := ""
	for i := 0; i < 320; i++ {
		longLocalPart += "a"
	}
	if rule.Validate(longLocalPart + "@example.com") {
		t.Fatalf("expected excessively long local part rejected at rfc5322")
	}
}

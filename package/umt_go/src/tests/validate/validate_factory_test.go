package validate_test

import (
	"testing"
	"time"

	umt_validate "github.com/riya-amemiya/umt-go/src/validate"
)

// --- Core helper tests (mirrors Validate/core/core.test.ts) ---

func TestCore_AppliesRulesAndMessages(t *testing.T) {
	validateString := umt_validate.Core[string]("string")

	minLength := umt_validate.ValidateReturnType[string]{
		TypeName: "string",
		Validate: func(v string) bool { return len(v) >= 3 },
		Message:  "String too short",
	}
	if !validateString("test", []umt_validate.ValidateReturnType[string]{minLength}, "").Validate {
		t.Fatalf("expected valid for length >= 3")
	}
	res := validateString("ab", []umt_validate.ValidateReturnType[string]{minLength}, "")
	if res.Validate {
		t.Fatalf("expected invalid for length < 3")
	}
	if res.Message != "String too short" {
		t.Fatalf("expected rule message, got %q", res.Message)
	}

	noMessage := umt_validate.ValidateReturnType[string]{
		TypeName: "string",
		Validate: func(v string) bool { return len(v) >= 3 },
	}
	if got := validateString("ab", []umt_validate.ValidateReturnType[string]{noMessage}, "").Message; got != "" {
		t.Fatalf("expected empty message for rule without message, got %q", got)
	}
}

func TestCore_MultipleRulesShortCircuit(t *testing.T) {
	validateString := umt_validate.Core[string]("string")
	minLength := umt_validate.ValidateReturnType[string]{
		Validate: func(v string) bool { return len(v) >= 3 },
		Message:  "String too short",
	}
	maxLength := umt_validate.ValidateReturnType[string]{
		Validate: func(v string) bool { return len(v) <= 10 },
		Message:  "String too long",
	}
	rules := []umt_validate.ValidateReturnType[string]{minLength, maxLength}

	if !validateString("test", rules, "").Validate {
		t.Fatalf("expected valid")
	}
	if got := validateString("ab", rules, "").Message; got != "String too short" {
		t.Fatalf("expected first failing rule message, got %q", got)
	}
	if got := validateString("very long string", rules, "").Message; got != "String too long" {
		t.Fatalf("expected second failing rule message, got %q", got)
	}
}

func TestCore_PreservesType(t *testing.T) {
	if got := umt_validate.Core[string]("string")("test", nil, "").Type; got != "test" {
		t.Fatalf("expected type carried through, got %q", got)
	}
}

// --- Any (mirrors Validate/any/core.test.ts) ---

func TestAny_AcceptsEverythingAndTags(t *testing.T) {
	v := umt_validate.Any[any]()
	for _, input := range []any{0, "", map[string]int{}, nil} {
		res := v(input)
		if !res.Validate {
			t.Fatalf("expected any to accept %v", input)
		}
		if res.Type != "any" {
			t.Fatalf("expected any tag, got %q", res.Type)
		}
	}
}

// --- Unknown (mirrors Validate/unknown/core.test.ts) ---

func TestUnknown_AcceptsEverythingAndTags(t *testing.T) {
	v := umt_validate.Unknown[any]()
	res := v(42)
	if !res.Validate || res.Type != "unknown" {
		t.Fatalf("expected unknown accept+tag, got %+v", res)
	}
}

// --- Never (mirrors Validate/never/core.test.ts) ---

func TestNever_RejectsEverything(t *testing.T) {
	v := umt_validate.Never[any]("")
	for _, input := range []any{0, "", nil, map[string]int{}} {
		if v(input).Validate {
			t.Fatalf("expected never to reject %v", input)
		}
	}
	if got := umt_validate.Never[any]("")(42).Type; got != "never" {
		t.Fatalf("expected never tag, got %q", got)
	}
}

func TestNever_CustomMessage(t *testing.T) {
	if got := umt_validate.Never[any]("must never be set")(0).Message; got != "must never be set" {
		t.Fatalf("expected custom message, got %q", got)
	}
}

// --- Boolean (mirrors Validate/boolean/core.test.ts via core) ---

func TestBoolean_AlwaysValid(t *testing.T) {
	if !umt_validate.ValidateBoolean(true, "").Validate {
		t.Fatalf("expected true valid")
	}
	if !umt_validate.ValidateBoolean(false, "").Validate {
		t.Fatalf("expected false valid")
	}
	v := umt_validate.BooleanValidator("")
	if !v(true).Validate || v(true).Type != true {
		t.Fatalf("expected validator valid + type carried")
	}
}

// --- Bigint (mirrors Validate/bigint/core.test.ts) ---

func TestBigint_AcceptsAndAppliesRules(t *testing.T) {
	if !umt_validate.ValidateBigint(0, nil, "").Validate {
		t.Fatalf("expected 0 valid")
	}
	positive := umt_validate.ValidateReturnType[int64]{
		Message:  "must be positive",
		Validate: func(v int64) bool { return v > 0 },
	}
	v := umt_validate.BigintValidator([]umt_validate.ValidateReturnType[int64]{positive}, "")
	if !v(1).Validate {
		t.Fatalf("expected 1 valid")
	}
	if v(0).Validate {
		t.Fatalf("expected 0 invalid under positive rule")
	}
	if got := v(-1).Message; got != "must be positive" {
		t.Fatalf("expected rule message, got %q", got)
	}
}

func TestBigint_EmptyMessageWhenRuleHasNone(t *testing.T) {
	positive := umt_validate.ValidateReturnType[int64]{Validate: func(v int64) bool { return v > 0 }}
	v := umt_validate.BigintValidator([]umt_validate.ValidateReturnType[int64]{positive}, "")
	if got := v(-1).Message; got != "" {
		t.Fatalf("expected empty message, got %q", got)
	}
}

// --- Date (mirrors Validate/date/core.test.ts) ---

func TestDate_AcceptsValidRejectsZero(t *testing.T) {
	valid := time.Date(2026, time.May, 7, 0, 0, 0, 0, time.UTC)
	if !umt_validate.ValidateDate(valid, "").Validate {
		t.Fatalf("expected valid date accepted")
	}
	if umt_validate.ValidateDate(time.Time{}, "").Validate {
		t.Fatalf("expected zero date rejected")
	}
}

func TestDate_CustomMessageOnFailure(t *testing.T) {
	v := umt_validate.DateValidator("must be a date")
	if got := v(time.Time{}).Message; got != "must be a date" {
		t.Fatalf("expected custom message, got %q", got)
	}
}

// --- File (mirrors Validate/file/core.test.ts) ---

func TestFile_AlwaysValid(t *testing.T) {
	if !umt_validate.ValidateFile([]byte("hello"), "").Validate {
		t.Fatalf("expected bytes valid")
	}
	v := umt_validate.FileValidator("")
	if !v([]byte{}).Validate {
		t.Fatalf("expected empty bytes valid")
	}
}

// --- Function (mirrors Validate/function/core.test.ts) ---

func TestValidateFunc_AlwaysValid(t *testing.T) {
	res := umt_validate.ValidateFunc(func(x int) int { return x + 1 }, "")
	if !res.Validate {
		t.Fatalf("expected function valid")
	}
	if res.Type(1) != 2 {
		t.Fatalf("expected carried function callable")
	}
}

func TestFuncImplement_ValidatesInputAndOutput(t *testing.T) {
	positive := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v > 0 },
		Message:  "must be positive",
	}
	output := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v > 0 },
	}
	wrapped := umt_validate.FuncImplement(func(n float64) float64 { return n * 2 }, &positive, &output)

	got, err := wrapped(2)
	if err != nil || got != 4 {
		t.Fatalf("expected (4, nil), got (%v, %v)", got, err)
	}
	if _, err := wrapped(-1); err == nil || err.Error() != "must be positive" {
		t.Fatalf("expected input rule message error, got %v", err)
	}
}

func TestFuncImplement_DefaultMessages(t *testing.T) {
	inputNoMsg := umt_validate.ValidateReturnType[float64]{Validate: func(v float64) bool { return false }}
	wrappedIn := umt_validate.FuncImplement(func(n float64) float64 { return n }, &inputNoMsg, nil)
	if _, err := wrappedIn(1); err == nil || err.Error() != "function input at index 0 failed validation" {
		t.Fatalf("expected default input message, got %v", err)
	}

	outNoMsg := umt_validate.ValidateReturnType[float64]{Validate: func(v float64) bool { return false }}
	wrappedOut := umt_validate.FuncImplement(func(n float64) float64 { return n }, nil, &outNoMsg)
	if _, err := wrappedOut(1); err == nil || err.Error() != "function output failed validation" {
		t.Fatalf("expected default output message, got %v", err)
	}
}

func TestFuncImplement_NoValidatorsPassThrough(t *testing.T) {
	wrapped := umt_validate.FuncImplement(func(n int) int { return n * 2 }, nil, nil)
	got, err := wrapped(3)
	if err != nil || got != 6 {
		t.Fatalf("expected (6, nil), got (%v, %v)", got, err)
	}
}

// --- InstanceOf (mirrors Validate/instanceof/core.test.ts) ---

type animal struct{ name string }
type other struct{}

func TestInstanceOf_ConcreteTypeIdentity(t *testing.T) {
	if !umt_validate.InstanceOf[animal](animal{name: "rex"}) {
		t.Fatalf("expected animal to be instance of animal")
	}
	if umt_validate.InstanceOf[other](animal{name: "rex"}) {
		t.Fatalf("expected animal not to be instance of other")
	}
	if umt_validate.InstanceOf[animal]("rex") {
		t.Fatalf("expected string not to be instance of animal")
	}
	if umt_validate.InstanceOf[animal](1) {
		t.Fatalf("expected number not to be instance of animal")
	}
}

// --- Number (mirrors Validate/number/core.test.ts) ---

func TestNumber_AppliesMinMaxRules(t *testing.T) {
	maxValue := umt_validate.ValidateReturnType[float64]{Validate: func(v float64) bool { return v <= 10 }}
	minValue := umt_validate.ValidateReturnType[float64]{
		Validate: func(v float64) bool { return v >= 3 },
		Message:  "Value must be between 3 and 10",
	}

	maxV := umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{maxValue}, "")
	if !maxV(5).Validate || maxV(11).Validate {
		t.Fatalf("expected max rule to accept 5 and reject 11")
	}

	bothV := umt_validate.NumberValidator([]umt_validate.ValidateReturnType[float64]{minValue, maxValue}, "")
	if !bothV(5).Validate || bothV(2).Validate || bothV(11).Validate {
		t.Fatalf("expected min/max combo behavior")
	}
	if got := bothV(2).Message; got != "Value must be between 3 and 10" {
		t.Fatalf("expected custom min message, got %q", got)
	}
}

// --- String (mirrors Validate/string/core.test.ts) ---

func TestString_AcceptsAndAppliesLengthRules(t *testing.T) {
	if !umt_validate.ValidateString("", nil, "").Validate {
		t.Fatalf("expected empty string valid")
	}
	// custom message cleared on success, mirroring TS runtime
	if got := umt_validate.ValidateString("", nil, "Must be string").Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}

	minLength := umt_validate.ValidateReturnType[string]{Validate: func(v string) bool { return len(v) >= 3 }}
	maxLength := umt_validate.ValidateReturnType[string]{Validate: func(v string) bool { return len(v) <= 5 }}
	v := umt_validate.StringValidator([]umt_validate.ValidateReturnType[string]{minLength, maxLength}, "")
	if v("ab").Validate {
		t.Fatalf("expected ab rejected")
	}
	if !v("abc").Validate || !v("abcde").Validate {
		t.Fatalf("expected abc and abcde accepted")
	}
	if v("abcdef").Validate {
		t.Fatalf("expected abcdef rejected")
	}
}

// --- TemplateLiteral (mirrors Validate/templateLiteral/core.test.ts) ---

func TestTemplateLiteral_StringFragment(t *testing.T) {
	v := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "hello, "},
		{Kind: umt_validate.TemplateLiteralStringTag},
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "!"},
	}, "")
	if !v("hello, world!").Validate || !v("hello, John!").Validate {
		t.Fatalf("expected matching strings accepted")
	}
	if v("hi, world!").Validate || v("hello, world").Validate {
		t.Fatalf("expected non-matching strings rejected")
	}
}

func TestTemplateLiteral_NumberBooleanBigint(t *testing.T) {
	num := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "item-"},
		{Kind: umt_validate.TemplateLiteralNumberTag},
	}, "")
	if !num("item-1").Validate || !num("item-100").Validate || !num("item-1.5").Validate {
		t.Fatalf("expected number suffixes accepted")
	}
	if num("item-abc").Validate {
		t.Fatalf("expected non-number suffix rejected")
	}

	flag := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "flag-"},
		{Kind: umt_validate.TemplateLiteralBooleanTag},
	}, "")
	if !flag("flag-true").Validate || !flag("flag-false").Validate || flag("flag-yes").Validate {
		t.Fatalf("expected boolean tag behavior")
	}

	big := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "big-"},
		{Kind: umt_validate.TemplateLiteralBigIntTag},
	}, "")
	if !big("big-1").Validate || !big("big-100").Validate || big("big-abc").Validate {
		t.Fatalf("expected bigint tag behavior")
	}
}

func TestTemplateLiteral_EscapesLiteralsAndMessage(t *testing.T) {
	v := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "[$]"},
		{Kind: umt_validate.TemplateLiteralStringTag},
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "."},
	}, "")
	if !v("[$]anything.").Validate {
		t.Fatalf("expected escaped literals matched verbatim")
	}
	if v("xanythingy").Validate {
		t.Fatalf("expected non-matching rejected")
	}

	msgV := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "user-"},
		{Kind: umt_validate.TemplateLiteralNumberTag},
	}, "must be user-<number>")
	if got := msgV("user-abc").Message; got != "must be user-<number>" {
		t.Fatalf("expected configured message, got %q", got)
	}
}

func TestTemplateLiteral_UnknownTagFallback(t *testing.T) {
	v := umt_validate.TemplateLiteral([]umt_validate.TemplateLiteralPart{
		{Kind: umt_validate.TemplateLiteralLiteral, Literal: "prefix-"},
		{Kind: umt_validate.TemplateLiteralUnknownTag},
	}, "")
	if !v("prefix-anything").Validate {
		t.Fatalf("expected permissive fragment to match non-empty suffix")
	}
	if v("noprefix").Validate {
		t.Fatalf("expected missing prefix rejected")
	}
}

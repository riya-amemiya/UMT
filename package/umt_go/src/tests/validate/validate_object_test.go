package validate_test

import (
	"testing"

	umt_validate "github.com/riya-amemiya/umt-go/src/validate"
)

// Helpers building the uniform any-accepting validators the composition layer
// consumes, mirroring the TS scalar factories used by the object tests.

func stringProp() umt_validate.PropertyValidator {
	return umt_validate.Property(umt_validate.StringValidator(nil, ""), "")
}

func stringPropMsg(message string) umt_validate.PropertyValidator {
	return umt_validate.Property(umt_validate.StringValidator(nil, message), message)
}

func numberProp() umt_validate.PropertyValidator {
	return umt_validate.Property(umt_validate.NumberValidator(nil, ""), "")
}

func numberPropMsg(message string) umt_validate.PropertyValidator {
	return umt_validate.Property(umt_validate.NumberValidator(nil, message), message)
}

func boolProp() umt_validate.PropertyValidator {
	return umt_validate.Property(umt_validate.BooleanValidator(""), "")
}

// any-layer scalar validators for union/intersection/nullable/optional.

func stringAny(message string) func(any) umt_validate.ValidateCoreReturnType[any] {
	return umt_validate.Adapt(umt_validate.StringValidator(nil, message), message)
}

func numberAny(message string) func(any) umt_validate.ValidateCoreReturnType[any] {
	return umt_validate.Adapt(umt_validate.NumberValidator(nil, message), message)
}

func boolAny(message string) func(any) umt_validate.ValidateCoreReturnType[any] {
	return umt_validate.Adapt(umt_validate.BooleanValidator(message), message)
}

// --- object core (mirrors object/core.test.ts) ---

func TestObject_EmptyShapeAcceptsEverything(t *testing.T) {
	validateObject := umt_validate.ObjectValidator(nil, "")
	invalid := map[string]any{"name": "John Doe", "age": "thirty"}
	if !validateObject.Validate(invalid).Validate {
		t.Fatalf("expected empty shape to accept any object")
	}
}

func TestObject_StringAndNumberTypes(t *testing.T) {
	validateObject := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringPropMsg("string"),
		"age":  numberPropMsg("number"),
		"array": umt_validate.PropertyAny(umt_validate.Adapt(
			umt_validate.ArrayOf(func(v any) umt_validate.ValidateCoreReturnType[any] {
				return umt_validate.Union[any, any](stringAny("string"), numberAny("number"))(v)
			}, "array"),
			"array",
		)),
	}, "object")

	valid := map[string]any{
		"name":  "John Doe",
		"age":   float64(30),
		"array": []any{"John Doe", float64(30)},
	}
	if got := validateObject.Validate(valid).Message; got != "" {
		t.Fatalf("expected empty message on success, got %q", got)
	}
	if !validateObject.Validate(valid).Validate {
		t.Fatalf("expected valid object accepted")
	}

	invalid := map[string]any{"name": "John Doe", "age": "thirty"}
	if validateObject.Validate(invalid).Validate {
		t.Fatalf("expected object with wrong age type rejected")
	}
}

func TestObject_CustomMessageOnNumberFailure(t *testing.T) {
	const customMessage = "Invalid object structure"
	validateObject := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  numberPropMsg(customMessage),
	}, "")

	invalid := map[string]any{"name": "John Doe", "age": "thirty"}
	result := validateObject.Validate(invalid)
	if result.Validate {
		t.Fatalf("expected rejection")
	}
	if result.Message != customMessage {
		t.Fatalf("expected custom property message %q, got %q", customMessage, result.Message)
	}
}

// --- intersection (mirrors object/intersection.test.ts) ---

func TestIntersection_AcceptsWhenAllPass(t *testing.T) {
	nameObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"name": stringProp()}, "")
	ageObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"age": numberProp()}, "")
	validate := umt_validate.Intersection(
		umt_validate.AdaptObject(nameObj.Validate, ""),
		umt_validate.AdaptObject(ageObj.Validate, ""),
	)

	valid := any(map[string]any{"name": "John", "age": float64(30)})
	if !validate(valid).Validate {
		t.Fatalf("expected valid")
	}
	if got := validate(valid).Message; got != "" {
		t.Fatalf("expected empty message, got %q", got)
	}
}

func TestIntersection_RejectsWhenAnyFails(t *testing.T) {
	nameObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"name": stringProp()}, "")
	ageObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"age": numberProp()}, "")
	validate := umt_validate.Intersection(
		umt_validate.AdaptObject(nameObj.Validate, ""),
		umt_validate.AdaptObject(ageObj.Validate, ""),
	)

	invalid := any(map[string]any{"name": "John"})
	if validate(invalid).Validate {
		t.Fatalf("expected rejection when age missing")
	}
}

func TestIntersection_FirstFailingMessage(t *testing.T) {
	nameObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"name": stringProp()}, "name validation failed")
	ageObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"age": numberProp()}, "age validation failed")
	validate := umt_validate.Intersection(
		umt_validate.AdaptObject(nameObj.Validate, "name validation failed"),
		umt_validate.AdaptObject(ageObj.Validate, "age validation failed"),
	)

	result := validate(any("not an object"))
	if result.Validate {
		t.Fatalf("expected rejection")
	}
	if result.Message != "name validation failed" {
		t.Fatalf("expected first failing message, got %q", result.Message)
	}
}

func TestIntersection_Nested(t *testing.T) {
	aStr := umt_validate.ObjectValidator(umt_validate.ObjectShape{"a": stringProp()}, "")
	bNum := umt_validate.ObjectValidator(umt_validate.ObjectShape{"b": numberProp()}, "")
	cStr := umt_validate.ObjectValidator(umt_validate.ObjectShape{"c": stringProp()}, "")
	inner := umt_validate.Intersection(
		umt_validate.AdaptObject(aStr.Validate, ""),
		umt_validate.AdaptObject(bNum.Validate, ""),
	)
	validate := umt_validate.Intersection(
		inner,
		umt_validate.AdaptObject(cStr.Validate, ""),
	)

	valid := any(map[string]any{"a": "x", "b": float64(1), "c": "y"})
	if !validate(valid).Validate {
		t.Fatalf("expected nested intersection accepted")
	}
}

// --- union (mirrors object/union.test.ts) ---

func TestUnion_AcceptsFirstMatch(t *testing.T) {
	validate := umt_validate.Union[any, any](stringAny(""), numberAny(""))
	if !validate("hello").Validate {
		t.Fatalf("expected string accepted")
	}
	if got := validate("hello").Message; got != "" {
		t.Fatalf("expected empty message, got %q", got)
	}
}

func TestUnion_AcceptsSecondMatch(t *testing.T) {
	validate := umt_validate.Union[any, any](stringAny(""), numberAny(""))
	if !validate(float64(42)).Validate {
		t.Fatalf("expected number accepted")
	}
}

func TestUnion_RejectsNoMatch(t *testing.T) {
	validate := umt_validate.Union[any, any](stringAny(""), numberAny(""))
	if validate(true).Validate {
		t.Fatalf("expected bool rejected")
	}
}

func TestUnion_ThreeOrMore(t *testing.T) {
	validate := umt_validate.Union[any, any](stringAny(""), numberAny(""), boolAny(""))
	if !validate("hello").Validate || !validate(float64(42)).Validate || !validate(true).Validate {
		t.Fatalf("expected all three member types accepted")
	}
	if validate(nil).Validate {
		t.Fatalf("expected nil rejected")
	}
}

func TestUnion_LastFailingMessage(t *testing.T) {
	validate := umt_validate.Union[any, any](stringAny("must be string"), numberAny("must be number"))
	result := validate(true)
	if result.Validate {
		t.Fatalf("expected rejection")
	}
	if result.Message != "must be number" {
		t.Fatalf("expected last failing message, got %q", result.Message)
	}
}

func TestUnion_NestedUnions(t *testing.T) {
	inner := umt_validate.Union[any, any](stringAny(""), numberAny(""))
	validate := umt_validate.Union[any, any](inner, boolAny(""))
	if !validate("hello").Validate || !validate(float64(42)).Validate || !validate(true).Validate {
		t.Fatalf("expected nested union to accept all member types")
	}
}

func TestUnion_NestedInsideObject(t *testing.T) {
	validateUser := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"id":   umt_validate.PropertyAny(umt_validate.Union[any, any](stringAny(""), numberAny(""))),
		"name": stringProp(),
	}, "")

	withString := map[string]any{"id": "abc", "name": "John"}
	withNumber := map[string]any{"id": float64(123), "name": "John"}
	if !validateUser.Validate(withString).Validate || !validateUser.Validate(withNumber).Validate {
		t.Fatalf("expected union property to accept string and number ids")
	}
}

// --- nullable (mirrors object/nullable.test.ts) ---

func TestNullable_NullTagAndPassThrough(t *testing.T) {
	validate := umt_validate.Nullable(numberAny(""))
	null := validate(nil)
	if !null.Validate || null.Type != "null" || null.Message != "" {
		t.Fatalf("expected passing null result with null tag, got %+v", null)
	}
	if !validate(float64(42)).Validate {
		t.Fatalf("expected wrapped number accepted")
	}
	if validate("not a number").Validate {
		t.Fatalf("expected non-number rejected")
	}
}

func TestNullable_InsideObjectAllowsNull(t *testing.T) {
	validateObject := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  umt_validate.PropertyAny(umt_validate.Nullable(numberAny(""))),
	}, "")

	withNull := map[string]any{"name": "John Doe", "age": nil}
	withAge := map[string]any{"name": "John Doe", "age": float64(30)}
	invalid := map[string]any{"name": "John Doe", "age": "not a number"}

	if !validateObject.Validate(withNull).Validate {
		t.Fatalf("expected null age accepted")
	}
	if !validateObject.Validate(withAge).Validate {
		t.Fatalf("expected numeric age accepted")
	}
	if validateObject.Validate(invalid).Validate {
		t.Fatalf("expected string age rejected")
	}
}

func TestNullable_WrapsUnion(t *testing.T) {
	validate := umt_validate.Nullable(umt_validate.Union[any, any](stringAny(""), numberAny("")))
	if !validate("hello").Validate || !validate(float64(42)).Validate || !validate(nil).Validate {
		t.Fatalf("expected union members and null accepted")
	}
	if validate(true).Validate {
		t.Fatalf("expected bool rejected")
	}
}

func TestNullable_WrapsIntersection(t *testing.T) {
	nameObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"name": stringProp()}, "")
	ageObj := umt_validate.ObjectValidator(umt_validate.ObjectShape{"age": numberProp()}, "")
	validate := umt_validate.Nullable(umt_validate.Intersection(
		umt_validate.AdaptObject(nameObj.Validate, ""),
		umt_validate.AdaptObject(ageObj.Validate, ""),
	))

	if !validate(map[string]any{"name": "John", "age": float64(30)}).Validate {
		t.Fatalf("expected intersected shape accepted")
	}
	if !validate(nil).Validate {
		t.Fatalf("expected null accepted")
	}
	if validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected missing age rejected")
	}
}

// --- optional (mirrors object/optional.test.ts) ---

func TestOptional_UndefinedTagAndPassThrough(t *testing.T) {
	validate := umt_validate.Optional(numberProp())
	absent := validate.Validate(nil)
	if !absent.Validate || absent.Type != "undefined" || absent.Message != "" {
		t.Fatalf("expected passing undefined result with undefined tag, got %+v", absent)
	}
	if !validate.Validate(float64(42)).Validate {
		t.Fatalf("expected wrapped number accepted")
	}
	if validate.Validate("not a number").Validate {
		t.Fatalf("expected non-number rejected")
	}
}

func TestOptional_InsideObjectAllowsOmission(t *testing.T) {
	validateObject := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  umt_validate.Optional(numberProp()),
	}, "")

	withoutAge := map[string]any{"name": "John Doe"}
	withAge := map[string]any{"name": "John Doe", "age": float64(30)}
	invalid := map[string]any{"name": "John Doe", "age": "not a number"}

	if !validateObject.Validate(withoutAge).Validate {
		t.Fatalf("expected omitted age accepted")
	}
	if !validateObject.Validate(withAge).Validate {
		t.Fatalf("expected numeric age accepted")
	}
	if validateObject.Validate(invalid).Validate {
		t.Fatalf("expected string age rejected")
	}
}

func TestOptionalNullable_BothBranches(t *testing.T) {
	validate := umt_validate.Optional(umt_validate.PropertyAny(umt_validate.Nullable(numberAny(""))))
	if !validate.Validate(float64(42)).Validate {
		t.Fatalf("expected number accepted")
	}
	if !validate.Validate(nil).Validate {
		t.Fatalf("expected nil (undefined branch) accepted")
	}
	if validate.Validate("nope").Validate {
		t.Fatalf("expected string rejected")
	}
}

func TestNullableOptional_BothBranches(t *testing.T) {
	validate := umt_validate.Nullable(umt_validate.Optional(numberProp()).Validate)
	if !validate(float64(42)).Validate {
		t.Fatalf("expected number accepted")
	}
	if !validate(nil).Validate {
		t.Fatalf("expected nil accepted")
	}
	if validate("nope").Validate {
		t.Fatalf("expected string rejected")
	}
}

// --- partial (mirrors object/partial.test.ts) ---

func TestPartial_MakesEveryPropertyOptional(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.Partial(base, "")

	empty := map[string]any{}
	partialData := map[string]any{"name": "John"}
	full := map[string]any{"name": "John", "age": float64(30), "active": true}

	if !validator.Validate(empty).Validate {
		t.Fatalf("expected empty accepted")
	}
	if !validator.Validate(partialData).Validate {
		t.Fatalf("expected partial accepted")
	}
	if !validator.Validate(full).Validate {
		t.Fatalf("expected full accepted")
	}
}

func TestPartial_ValidatesSuppliedProperties(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  numberProp(),
	}, "")
	validator := umt_validate.Partial(base, "")
	if validator.Validate(map[string]any{"age": "30"}).Validate {
		t.Fatalf("expected wrong-typed age rejected")
	}
}

func TestPartial_Idempotent(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  numberProp(),
	}, "")
	validator := umt_validate.Partial(umt_validate.Partial(base, ""), "")
	if !validator.Validate(map[string]any{}).Validate {
		t.Fatalf("expected empty accepted after double partial")
	}
	if validator.Validate(map[string]any{"age": "30"}).Validate {
		t.Fatalf("expected wrong-typed age still rejected after double partial")
	}
}

func TestPartial_ComposesWithNullable(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  numberProp(),
	}, "")
	validator := umt_validate.Nullable(umt_validate.AdaptObject(umt_validate.Partial(base, "").Validate, ""))
	if !validator(nil).Validate {
		t.Fatalf("expected null accepted")
	}
	if !validator(map[string]any{}).Validate {
		t.Fatalf("expected empty accepted")
	}
	if !validator(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected partial accepted")
	}
}

// --- required (mirrors object/required.test.ts) ---

func TestRequired_RequiresAllKeys(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   umt_validate.Optional(stringProp()),
		"age":    umt_validate.Optional(numberProp()),
		"active": umt_validate.Optional(boolProp()),
	}, "")
	validator := umt_validate.Required(base, "")

	valid := map[string]any{"name": "John", "age": float64(30), "active": true}
	if !validator.Validate(valid).Validate {
		t.Fatalf("expected all-present accepted")
	}
	if validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected missing required keys rejected")
	}
}

func TestRequired_UndoesPartial(t *testing.T) {
	original := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  numberProp(),
	}, "")
	validator := umt_validate.Required(umt_validate.Partial(original, ""), "")

	valid := map[string]any{"name": "John", "age": float64(30)}
	if !validator.Validate(valid).Validate {
		t.Fatalf("expected all-present accepted after required(partial)")
	}
	if validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected missing age rejected after required(partial)")
	}
}

func TestRequired_KeepsNonOptionalUnchanged(t *testing.T) {
	mixed := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
		"age":  umt_validate.Optional(numberProp()),
	}, "")
	validator := umt_validate.Required(mixed, "")

	valid := map[string]any{"name": "John", "age": float64(30)}
	if !validator.Validate(valid).Validate {
		t.Fatalf("expected all-present accepted")
	}
	if validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected missing age rejected")
	}
}

// --- omitKeys (mirrors object/omitKeys.test.ts) ---

func TestOmitKeys_RemainingKeys(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.OmitKeys(base, []string{"active"}, "")

	valid := map[string]any{"name": "John", "age": float64(30)}
	if !validator.Validate(valid).Validate {
		t.Fatalf("expected remaining keys validated")
	}
}

func TestOmitKeys_IgnoresUnknownAndDroppedKeys(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.OmitKeys(base, []string{"age", "active"}, "")
	if !validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected only name validated")
	}
}

func TestOmitKeys_RejectsRemainingFailures(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.OmitKeys(base, []string{"active"}, "")
	if validator.Validate(map[string]any{"name": "John", "age": "30"}).Validate {
		t.Fatalf("expected wrong-typed age rejected")
	}
}

// --- pickKeys (mirrors object/pickKeys.test.ts) ---

func TestPickKeys_PickedKeys(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.PickKeys(base, []string{"name", "age"}, "")

	valid := map[string]any{"name": "John", "age": float64(30)}
	if !validator.Validate(valid).Validate {
		t.Fatalf("expected picked keys validated")
	}
}

func TestPickKeys_IgnoresUnpicked(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.PickKeys(base, []string{"name"}, "")
	if !validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected only name validated")
	}
}

func TestPickKeys_RejectsPickedFailures(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name":   stringProp(),
		"age":    numberProp(),
		"active": boolProp(),
	}, "")
	validator := umt_validate.PickKeys(base, []string{"age"}, "")
	if validator.Validate(map[string]any{"age": "30"}).Validate {
		t.Fatalf("expected wrong-typed age rejected")
	}
}

func TestPickKeys_AbsentKeySkipped(t *testing.T) {
	base := umt_validate.ObjectValidator(umt_validate.ObjectShape{
		"name": stringProp(),
	}, "")
	validator := umt_validate.PickKeys(base, []string{"name", "missing"}, "")
	if !validator.Validate(map[string]any{"name": "John"}).Validate {
		t.Fatalf("expected absent picked key skipped")
	}
}

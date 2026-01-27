package validate_test

import (
	"math"
	"testing"

	"github.com/riya-amemiya/umt-go/src/validate"
)

// --- IsNumber tests ---

func TestIsNumber_IntegersAndDecimals(t *testing.T) {
	cases := []struct {
		input any
		want  bool
	}{
		{5, true},
		{-5, true},
		{0, true},
		{5.5, true},
		{-5.5, true},
		{0.0, true},
		{int8(1), true},
		{int16(1), true},
		{int32(1), true},
		{int64(1), true},
		{uint(1), true},
		{uint8(1), true},
		{uint16(1), true},
		{uint32(1), true},
		{uint64(1), true},
		{float32(1.5), true},
		{float64(1.5), true},
	}
	for _, tc := range cases {
		if got := validate.IsNumber(tc.input); got != tc.want {
			t.Errorf("IsNumber(%v) = %v, want %v", tc.input, got, tc.want)
		}
	}
}

func TestIsNumber_NonNumberTypes(t *testing.T) {
	cases := []struct {
		input any
		want  bool
	}{
		{nil, false},
		{"hello", false},
		{true, false},
		{false, false},
		{[]int{1, 2}, false},
		{map[string]int{"a": 1}, false},
	}
	for _, tc := range cases {
		if got := validate.IsNumber(tc.input); got != tc.want {
			t.Errorf("IsNumber(%v) = %v, want %v", tc.input, got, tc.want)
		}
	}
}

// --- IsString tests ---

func TestIsString(t *testing.T) {
	cases := []struct {
		input any
		want  bool
	}{
		{"", true},
		{"test", true},
		{"hello world", true},
		{"123", true},
		{123, false},
		{0, false},
		{-1, false},
		{3.14, false},
		{nil, false},
		{true, false},
		{false, false},
		{[]int{}, false},
		{map[string]int{}, false},
	}
	for _, tc := range cases {
		if got := validate.IsString(tc.input); got != tc.want {
			t.Errorf("IsString(%v) = %v, want %v", tc.input, got, tc.want)
		}
	}
}

// --- IsArray tests ---

func TestIsArray(t *testing.T) {
	cases := []struct {
		name  string
		input any
		want  bool
	}{
		{"int slice", []int{1, 2, 3}, true},
		{"string slice", []string{"a", "b", "c"}, true},
		{"bool slice", []bool{true, false}, true},
		{"empty slice", []int{}, true},
		{"int value", 1, false},
		{"string value", "hello", false},
		{"bool value", true, false},
		{"map value", map[string]int{"a": 1}, false},
		{"nil value", nil, false},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			if got := validate.IsArray(tc.input); got != tc.want {
				t.Errorf("IsArray(%v) = %v, want %v", tc.input, got, tc.want)
			}
		})
	}
}

// --- IsDouble tests ---

func TestIsDouble(t *testing.T) {
	cases := []struct {
		input float64
		want  bool
	}{
		{1.5, true},
		{-1.5, true},
		{1.23e-4, true},
		{1.0, false},
		{0.0, false},
		{-1.0, false},
		{math.NaN(), false},
		{math.Inf(1), false},
		{math.Inf(-1), false},
		{18.0, false}, // 0x12 = 18, integer
	}
	for _, tc := range cases {
		if got := validate.IsDouble(tc.input); got != tc.want {
			t.Errorf("IsDouble(%v) = %v, want %v", tc.input, got, tc.want)
		}
	}
}

// --- IsEqual tests ---

func TestIsEqual(t *testing.T) {
	t.Run("primitive values", func(t *testing.T) {
		if !validate.IsEqual(1, 1) {
			t.Error("IsEqual(1, 1) should be true")
		}
		if !validate.IsEqual("test", "test") {
			t.Error(`IsEqual("test", "test") should be true`)
		}
		if !validate.IsEqual(true, true) {
			t.Error("IsEqual(true, true) should be true")
		}
		if !validate.IsEqual(nil, nil) {
			t.Error("IsEqual(nil, nil) should be true")
		}
	})

	t.Run("different values", func(t *testing.T) {
		if validate.IsEqual(1, 2) {
			t.Error("IsEqual(1, 2) should be false")
		}
		if validate.IsEqual("test", "other") {
			t.Error(`IsEqual("test", "other") should be false`)
		}
		if validate.IsEqual(true, false) {
			t.Error("IsEqual(true, false) should be false")
		}
	})
}

// --- IsDeepEqual tests ---

func TestIsDeepEqual(t *testing.T) {
	t.Run("primitive values", func(t *testing.T) {
		if !validate.IsDeepEqual(1, 1) {
			t.Error("IsDeepEqual(1, 1) should be true")
		}
		if !validate.IsDeepEqual("test", "test") {
			t.Error(`IsDeepEqual("test", "test") should be true`)
		}
		if !validate.IsDeepEqual(nil, nil) {
			t.Error("IsDeepEqual(nil, nil) should be true")
		}
	})

	t.Run("different primitive values", func(t *testing.T) {
		if validate.IsDeepEqual(1, 2) {
			t.Error("IsDeepEqual(1, 2) should be false")
		}
		if validate.IsDeepEqual("test", "other") {
			t.Error(`IsDeepEqual("test", "other") should be false`)
		}
	})

	t.Run("slices", func(t *testing.T) {
		if !validate.IsDeepEqual([]int{1, 2, 3}, []int{1, 2, 3}) {
			t.Error("IsDeepEqual([1,2,3], [1,2,3]) should be true")
		}
		if validate.IsDeepEqual([]int{1, 2, 3}, []int{1, 2, 4}) {
			t.Error("IsDeepEqual([1,2,3], [1,2,4]) should be false")
		}
		if validate.IsDeepEqual([]int{1, 2, 3}, []int{1, 2, 3, 4}) {
			t.Error("IsDeepEqual([1,2,3], [1,2,3,4]) should be false")
		}
	})

	t.Run("maps", func(t *testing.T) {
		m1 := map[string]int{"a": 1, "b": 2}
		m2 := map[string]int{"a": 1, "b": 2}
		m3 := map[string]int{"a": 1, "b": 3}
		if !validate.IsDeepEqual(m1, m2) {
			t.Error("maps with same content should be equal")
		}
		if validate.IsDeepEqual(m1, m3) {
			t.Error("maps with different values should not be equal")
		}
	})

	t.Run("nested structures", func(t *testing.T) {
		type inner struct {
			C []int
		}
		type outer struct {
			A inner
		}
		o1 := outer{A: inner{C: []int{1, 2, 3}}}
		o2 := outer{A: inner{C: []int{1, 2, 3}}}
		o3 := outer{A: inner{C: []int{1, 2, 4}}}
		if !validate.IsDeepEqual(o1, o2) {
			t.Error("nested structs with same content should be equal")
		}
		if validate.IsDeepEqual(o1, o3) {
			t.Error("nested structs with different content should not be equal")
		}
	})
}

// --- IsPrimeNumber tests ---

func TestIsPrimeNumber(t *testing.T) {
	t.Run("numbers less than or equal to 1", func(t *testing.T) {
		if validate.IsPrimeNumber(0) {
			t.Error("0 should not be prime")
		}
		if validate.IsPrimeNumber(1) {
			t.Error("1 should not be prime")
		}
	})

	t.Run("prime numbers", func(t *testing.T) {
		primes := []int{2, 3, 5, 7, 11, 13, 17, 19, 23, 29}
		for _, p := range primes {
			if !validate.IsPrimeNumber(p) {
				t.Errorf("%d should be prime", p)
			}
		}
	})

	t.Run("composite numbers", func(t *testing.T) {
		composites := []int{4, 6, 8, 9, 10, 12, 14, 15, 16, 18}
		for _, c := range composites {
			if validate.IsPrimeNumber(c) {
				t.Errorf("%d should not be prime", c)
			}
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		negatives := []int{-2, -7, -11}
		for _, n := range negatives {
			if validate.IsPrimeNumber(n) {
				t.Errorf("%d should not be prime", n)
			}
		}
	})

	t.Run("large prime number", func(t *testing.T) {
		if !validate.IsPrimeNumber(982451653) {
			t.Error("982451653 should be prime")
		}
	})

	t.Run("large non-prime number", func(t *testing.T) {
		if validate.IsPrimeNumber(10000000000000) {
			t.Error("10^13 should not be prime")
		}
	})
}

// --- IsPerfectSquare tests ---

func TestIsPerfectSquare(t *testing.T) {
	t.Run("perfect squares", func(t *testing.T) {
		squares := []int{0, 1, 4, 9, 16, 25, 49, 100000000}
		for _, s := range squares {
			if !validate.IsPerfectSquare(s) {
				t.Errorf("%d should be a perfect square", s)
			}
		}
	})

	t.Run("non-perfect squares", func(t *testing.T) {
		nonSquares := []int{2, 3, 5, 20, 100000002}
		for _, s := range nonSquares {
			if validate.IsPerfectSquare(s) {
				t.Errorf("%d should not be a perfect square", s)
			}
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		if validate.IsPerfectSquare(-16) {
			t.Error("-16 should not be a perfect square")
		}
	})
}

// --- IsValueNaN tests ---

func TestIsValueNaN(t *testing.T) {
	t.Run("NaN", func(t *testing.T) {
		if !validate.IsValueNaN(math.NaN()) {
			t.Error("NaN should be NaN")
		}
	})

	t.Run("valid numbers", func(t *testing.T) {
		if validate.IsValueNaN(0) {
			t.Error("0 should not be NaN")
		}
		if validate.IsValueNaN(1) {
			t.Error("1 should not be NaN")
		}
		if validate.IsValueNaN(-1) {
			t.Error("-1 should not be NaN")
		}
		if validate.IsValueNaN(math.Inf(1)) {
			t.Error("Inf should not be NaN")
		}
	})
}

// --- IsDictionaryObject tests ---

func TestIsDictionaryObject(t *testing.T) {
	cases := []struct {
		name  string
		input any
		want  bool
	}{
		{"empty map", map[string]any{}, true},
		{"map with strings", map[string]string{"foo": "bar"}, true},
		{"map with ints", map[string]int{"foo": 1}, true},
		{"map with mixed", map[string]any{"foo": "bar", "baz": 1, "qux": true}, true},
		{"slice", []int{1, 2, 3}, false},
		{"string", "foo", false},
		{"int", 42, false},
		{"bool", true, false},
		{"nil", nil, false},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			if got := validate.IsDictionaryObject(tc.input); got != tc.want {
				t.Errorf("IsDictionaryObject(%v) = %v, want %v", tc.input, got, tc.want)
			}
		})
	}
}

// --- IsNotEmpty tests ---

func TestIsNotEmpty(t *testing.T) {
	t.Run("non-empty map", func(t *testing.T) {
		m := map[string]int{"a": 1}
		if !validate.IsNotEmpty(m) {
			t.Error("non-empty map should not be empty")
		}
	})

	t.Run("empty map", func(t *testing.T) {
		m := map[string]int{}
		if validate.IsNotEmpty(m) {
			t.Error("empty map should be empty")
		}
	})

	t.Run("non-empty slice", func(t *testing.T) {
		s := []int{1, 2, 3}
		if !validate.IsNotEmpty(s) {
			t.Error("non-empty slice should not be empty")
		}
	})

	t.Run("empty slice", func(t *testing.T) {
		s := []int{}
		if validate.IsNotEmpty(s) {
			t.Error("empty slice should be empty")
		}
	})

	t.Run("non-empty string", func(t *testing.T) {
		if !validate.IsNotEmpty("hello") {
			t.Error("non-empty string should not be empty")
		}
	})

	t.Run("empty string", func(t *testing.T) {
		if validate.IsNotEmpty("") {
			t.Error("empty string should be empty")
		}
	})

	t.Run("nil", func(t *testing.T) {
		if validate.IsNotEmpty(nil) {
			t.Error("nil should be empty")
		}
	})
}

// --- ParseEmail tests ---

func TestParseEmail(t *testing.T) {
	t.Run("valid emails", func(t *testing.T) {
		validEmails := []string{
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
		for _, email := range validEmails {
			parts, err := validate.ParseEmail(email)
			if err != nil {
				t.Errorf("ParseEmail(%q) returned error: %v", email, err)
			}
			if parts.Local == "" || parts.Domain == "" {
				t.Errorf("ParseEmail(%q) returned empty parts: %+v", email, parts)
			}
		}
	})

	t.Run("invalid emails", func(t *testing.T) {
		invalidEmails := []string{
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
		}
		for _, email := range invalidEmails {
			_, err := validate.ParseEmail(email)
			if err == nil {
				t.Errorf("ParseEmail(%q) should have returned error", email)
			}
		}
	})

	t.Run("extracts parts correctly", func(t *testing.T) {
		parts, err := validate.ParseEmail("user@example.com")
		if err != nil {
			t.Fatalf("ParseEmail returned error: %v", err)
		}
		if parts.Local != "user" {
			t.Errorf("expected local='user', got '%s'", parts.Local)
		}
		if parts.Domain != "example.com" {
			t.Errorf("expected domain='example.com', got '%s'", parts.Domain)
		}

		parts2, err := validate.ParseEmail("test.user@sub.example.com")
		if err != nil {
			t.Fatalf("ParseEmail returned error: %v", err)
		}
		if parts2.Local != "test.user" {
			t.Errorf("expected local='test.user', got '%s'", parts2.Local)
		}
		if parts2.Domain != "sub.example.com" {
			t.Errorf("expected domain='sub.example.com', got '%s'", parts2.Domain)
		}
	})

	t.Run("domain case preserved", func(t *testing.T) {
		parts, err := validate.ParseEmail("user@EXAMPLE.COM")
		if err != nil {
			t.Fatalf("ParseEmail returned error: %v", err)
		}
		if parts.Domain != "EXAMPLE.COM" {
			t.Errorf("expected domain='EXAMPLE.COM', got '%s'", parts.Domain)
		}
	})
}

// --- ParseEmailWithLevel tests ---

func TestParseEmailWithLevel(t *testing.T) {
	t.Run("basic level", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@example.com", "basic")
		if !result.Valid {
			t.Error("user@example.com should be valid at basic level")
		}
		if result.Parts == nil || result.Parts.Local != "user" || result.Parts.Domain != "example.com" {
			t.Errorf("unexpected parts: %+v", result.Parts)
		}
	})

	t.Run("validates with different RFC levels", func(t *testing.T) {
		levels := []string{"basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"}
		for _, level := range levels {
			result := validate.ParseEmailWithLevel("test@example.com", level)
			if !result.Valid {
				t.Errorf("test@example.com should be valid at %s level", level)
			}
		}
	})

	t.Run("basic level rejects invalid", func(t *testing.T) {
		invalidEmails := []string{
			"plainaddress",
			"@example.com",
			"user@",
			"user@@example.com",
			"user name@example.com",
			"",
		}
		for _, email := range invalidEmails {
			result := validate.ParseEmailWithLevel(email, "basic")
			if result.Valid {
				t.Errorf("%q should be invalid at basic level", email)
			}
		}
	})

	t.Run("rfc822 allows domains without TLD", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@localhost", "rfc822")
		if !result.Valid {
			t.Error("user@localhost should be valid at rfc822 level")
		}
		result = validate.ParseEmailWithLevel("user@localhost", "rfc5321")
		if result.Valid {
			t.Error("user@localhost should be invalid at rfc5321 level")
		}
	})

	t.Run("rfc2822 requires TLD", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@localhost", "rfc2822")
		if result.Valid {
			t.Error("user@localhost should be invalid at rfc2822 level")
		}
		result = validate.ParseEmailWithLevel("user@example.com", "rfc2822")
		if !result.Valid {
			t.Error("user@example.com should be valid at rfc2822 level")
		}
	})

	t.Run("rfc2822 rejects consecutive dots", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user..name@example.com", "rfc2822")
		if result.Valid {
			t.Error("email with consecutive dots should be invalid at rfc2822")
		}
	})

	t.Run("rfc5321 rejects domain with special chars", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@exam!ple.com", "rfc5321")
		if result.Valid {
			t.Error("domain with ! should be invalid at rfc5321")
		}
		result = validate.ParseEmailWithLevel("user@exam#ple.com", "rfc5321")
		if result.Valid {
			t.Error("domain with # should be invalid at rfc5321")
		}
	})

	t.Run("rfc5321 domain literals", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@[192.168.0.1]", "rfc5321")
		if !result.Valid {
			t.Error("user@[192.168.0.1] should be valid at rfc5321")
		}
		result = validate.ParseEmailWithLevel("user@[IPv6:2001:db8::1]", "rfc5321")
		if !result.Valid {
			t.Error("IPv6 domain literal should be valid at rfc5321")
		}
	})

	t.Run("rfc5322 allows comments", func(t *testing.T) {
		commentEmails := []string{
			"user(comment)@example.com",
			"user@(comment)example.com",
			"(comment)user@example.com",
		}
		for _, email := range commentEmails {
			result := validate.ParseEmailWithLevel(email, "rfc5322")
			if !result.Valid {
				t.Errorf("%q should be valid at rfc5322 level", email)
			}
			resultBasic := validate.ParseEmailWithLevel(email, "basic")
			if resultBasic.Valid {
				t.Errorf("%q should be invalid at basic level", email)
			}
		}
	})

	t.Run("rfc5322 quoted strings", func(t *testing.T) {
		quotedEmails := []string{
			`"user with space"@example.com`,
			`"user..dots"@example.com`,
			`".leadingdot"@example.com`,
			`"trailingdot."@example.com`,
		}
		for _, email := range quotedEmails {
			result := validate.ParseEmailWithLevel(email, "rfc5322")
			if !result.Valid {
				t.Errorf("%q should be valid at rfc5322 level", email)
			}
			resultBasic := validate.ParseEmailWithLevel(email, "basic")
			if resultBasic.Valid {
				t.Errorf("%q should be invalid at basic level", email)
			}
		}
	})

	t.Run("rejects completely invalid strings across all levels", func(t *testing.T) {
		invalidStrings := []string{
			"not-an-email-at-all",
			"12345",
			"random string with spaces",
			"just-text",
			"null",
			"undefined",
			"true",
			"false",
			"0",
			"NaN",
		}
		levels := []string{"basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"}
		for _, str := range invalidStrings {
			for _, level := range levels {
				result := validate.ParseEmailWithLevel(str, level)
				if result.Valid {
					t.Errorf("%q should be invalid at %s level", str, level)
				}
			}
		}
	})

	t.Run("rejects malformed domains across all levels", func(t *testing.T) {
		malformedDomains := []string{
			"user@",
			"user@.",
			"user@..",
			"user@...",
			"user@.domain.example.com",
		}
		levels := []string{"basic", "rfc822", "rfc2822", "rfc5321", "rfc5322"}
		for _, email := range malformedDomains {
			for _, level := range levels {
				result := validate.ParseEmailWithLevel(email, level)
				if result.Valid {
					t.Errorf("%q should be invalid at %s level", email, level)
				}
			}
		}
	})

	t.Run("rfc5322 domain literals", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@[192.168.1.1]", "rfc5322")
		if !result.Valid {
			t.Error("IP literal should be valid at rfc5322")
		}
		result = validate.ParseEmailWithLevel("user@[IPv6:2001:db8::1]", "rfc5322")
		if !result.Valid {
			t.Error("IPv6 literal should be valid at rfc5322")
		}
	})

	t.Run("basic rejects IP literals", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@[192.168.0.1]", "basic")
		if result.Valid {
			t.Error("IP literal should be invalid at basic level")
		}
	})

	t.Run("rfc5322 empty quoted string", func(t *testing.T) {
		result := validate.ParseEmailWithLevel(`""@example.com`, "rfc5322")
		if !result.Valid {
			t.Error(`""@example.com should be valid at rfc5322`)
		}
	})

	t.Run("punycode addresses", func(t *testing.T) {
		result := validate.ParseEmailWithLevel("user@xn--bcher-kva.com", "basic")
		if !result.Valid {
			t.Error("punycode address should be valid at basic level")
		}
		result = validate.ParseEmailWithLevel("user@xn--bcher-kva.com", "rfc5321")
		if !result.Valid {
			t.Error("punycode address should be valid at rfc5321 level")
		}
	})
}

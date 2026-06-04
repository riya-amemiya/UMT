package validate

import "time"

// ValidateDate validates a date value directly. It mirrors the TypeScript
// `date(message?)` runtime (Validate/date/core.ts), which accepts only values
// that are a Date instance and represent a valid moment in time (rejecting
// `new Date("not-a-date")` whose timestamp is NaN), and the Rust
// `umt_validate_date` / `umt_date_validator` pair.
//
// In Go a date is modeled as time.Time. The analog of an invalid JavaScript Date
// is the zero value time.Time{} (mirroring how the Rust port models the invalid
// case with None): a zero time.Time is reported as invalid, any other time.Time
// is reported as valid.
func ValidateDate(value time.Time, message string) ValidateCoreReturnType[time.Time] {
	if value.IsZero() {
		return ValidateCoreReturnType[time.Time]{
			Validate: false,
			Message:  message,
			Type:     value,
		}
	}
	return ValidateCoreReturnType[time.Time]{
		Validate: true,
		Message:  "",
		Type:     value,
	}
}

// DateValidator creates a date validator function, mirroring the TypeScript
// `date(message?)` factory and the Rust `umt_date_validator`. A zero time.Time
// is rejected; any other time.Time is accepted.
func DateValidator(message string) func(value time.Time) ValidateCoreReturnType[time.Time] {
	return func(value time.Time) ValidateCoreReturnType[time.Time] {
		return ValidateDate(value, message)
	}
}

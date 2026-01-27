package errutil_test

import (
	"errors"
	"fmt"
	"testing"
	"time"

	"github.com/riya-amemiya/umt-go/src/errutil"
)

// ---------------------------------------------------------------------------
// Retry tests
// ---------------------------------------------------------------------------

func TestRetrySucceedsOnFirstAttempt(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		return nil
	}, 3, time.Millisecond)

	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if callCount != 1 {
		t.Errorf("expected 1 call, got %d", callCount)
	}
}

func TestRetryExhaustsAllRetries(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		return fmt.Errorf("test error")
	}, 3, time.Millisecond)

	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if err.Error() != "test error" {
		t.Errorf("expected 'test error', got %q", err.Error())
	}
	// 1 initial + 3 retries = 4 total calls
	if callCount != 4 {
		t.Errorf("expected 4 calls, got %d", callCount)
	}
}

func TestRetrySucceedsOnSecondAttempt(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		if callCount == 1 {
			return fmt.Errorf("first failure")
		}
		return nil
	}, 3, time.Millisecond)

	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if callCount != 2 {
		t.Errorf("expected 2 calls, got %d", callCount)
	}
}

func TestRetrySucceedsOnLastAttempt(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		if callCount <= 3 {
			return fmt.Errorf("failure %d", callCount)
		}
		return nil
	}, 3, time.Millisecond)

	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if callCount != 4 {
		t.Errorf("expected 4 calls, got %d", callCount)
	}
}

func TestRetryWithZeroRetries(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		return fmt.Errorf("test error")
	}, 0, time.Millisecond)

	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if callCount != 1 {
		t.Errorf("expected 1 call, got %d", callCount)
	}
}

func TestRetryWithOneRetry(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		return fmt.Errorf("test error")
	}, 1, time.Millisecond)

	if err == nil {
		t.Fatal("expected error, got nil")
	}
	// 1 initial + 1 retry = 2 total calls
	if callCount != 2 {
		t.Errorf("expected 2 calls, got %d", callCount)
	}
}

func TestRetryRespectsDelay(t *testing.T) {
	callCount := 0
	start := time.Now()
	_ = errutil.Retry(func() error {
		callCount++
		if callCount <= 2 {
			return fmt.Errorf("failure")
		}
		return nil
	}, 3, 10*time.Millisecond)

	elapsed := time.Since(start)
	// Should have at least 2 delays of 10ms each (failed twice then succeeded)
	if elapsed < 15*time.Millisecond {
		t.Errorf("expected at least ~20ms delay, but only %v elapsed", elapsed)
	}
}

func TestRetryReturnsLastError(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		return fmt.Errorf("error attempt %d", callCount)
	}, 2, time.Millisecond)

	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if err.Error() != "error attempt 3" {
		t.Errorf("expected 'error attempt 3', got %q", err.Error())
	}
}

func TestRetryMixedSuccessAndFailure(t *testing.T) {
	callCount := 0
	err := errutil.Retry(func() error {
		callCount++
		if callCount == 1 {
			return fmt.Errorf("temp failure")
		}
		return nil
	}, 3, time.Millisecond)

	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if callCount != 2 {
		t.Errorf("expected 2 calls, got %d", callCount)
	}
}

// ---------------------------------------------------------------------------
// SafeExecute tests
// ---------------------------------------------------------------------------

func TestSafeExecuteSuccessString(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		return "test", nil
	})
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if result != "test" {
		t.Errorf("expected 'test', got %v", result)
	}
}

func TestSafeExecuteSuccessNumber(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		return 42, nil
	})
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if result != 42 {
		t.Errorf("expected 42, got %v", result)
	}
}

func TestSafeExecuteSuccessObject(t *testing.T) {
	type obj struct {
		Key string
	}
	expected := obj{Key: "value"}
	result, err := errutil.SafeExecute(func() (any, error) {
		return expected, nil
	})
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	got, ok := result.(obj)
	if !ok {
		t.Fatalf("expected obj type, got %T", result)
	}
	if got.Key != "value" {
		t.Errorf("expected Key='value', got %q", got.Key)
	}
}

func TestSafeExecuteReturnsError(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		return nil, fmt.Errorf("test error")
	})
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if err.Error() != "test error" {
		t.Errorf("expected 'test error', got %q", err.Error())
	}
	if result != nil {
		t.Errorf("expected nil result, got %v", result)
	}
}

func TestSafeExecuteCatchesPanicWithError(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		panic(errors.New("panic error"))
	})
	if err == nil {
		t.Fatal("expected error from panic, got nil")
	}
	if err.Error() != "panic error" {
		t.Errorf("expected 'panic error', got %q", err.Error())
	}
	if result != nil {
		t.Errorf("expected nil result, got %v", result)
	}
}

func TestSafeExecuteCatchesPanicWithString(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		panic("string panic")
	})
	if err == nil {
		t.Fatal("expected error from panic, got nil")
	}
	if err.Error() != "string panic" {
		t.Errorf("expected 'string panic', got %q", err.Error())
	}
	if result != nil {
		t.Errorf("expected nil result, got %v", result)
	}
}

func TestSafeExecuteCatchesPanicWithCustomType(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		panic(42)
	})
	if err == nil {
		t.Fatal("expected error from panic, got nil")
	}
	if err.Error() != "42" {
		t.Errorf("expected '42', got %q", err.Error())
	}
	if result != nil {
		t.Errorf("expected nil result, got %v", result)
	}
}

func TestSafeExecuteSuccessWithNilResult(t *testing.T) {
	result, err := errutil.SafeExecute(func() (any, error) {
		return nil, nil
	})
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}
	if result != nil {
		t.Errorf("expected nil result, got %v", result)
	}
}

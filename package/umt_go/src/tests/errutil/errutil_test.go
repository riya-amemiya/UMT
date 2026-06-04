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

// ---------------------------------------------------------------------------
// SuccessFunction / ErrorFunction tests
// ---------------------------------------------------------------------------

func TestSuccessFunctionCreatesSuccessResult(t *testing.T) {
	result := errutil.SuccessFunction[int, error](42)
	if result.Type != "success" {
		t.Errorf("expected type 'success', got %q", result.Type)
	}
	if result.Value != 42 {
		t.Errorf("expected value 42, got %v", result.Value)
	}
}

func TestErrorFunctionCreatesErrorResult(t *testing.T) {
	wrapped := errors.New("test")
	result := errutil.ErrorFunction[int](wrapped)
	if result.Type != "error" {
		t.Errorf("expected type 'error', got %q", result.Type)
	}
	if result.Error == nil {
		t.Fatal("expected non-nil error")
	}
	if result.Error.Error() != "test" {
		t.Errorf("expected 'test', got %q", result.Error.Error())
	}
}

// ---------------------------------------------------------------------------
// SafeExecuteResult tests
// ---------------------------------------------------------------------------

func TestSafeExecuteResultSuccessString(t *testing.T) {
	result := errutil.SafeExecuteResult(func() string { return "test" })
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != "test" {
		t.Errorf("expected 'test', got %v", result.Value)
	}
}

func TestSafeExecuteResultSuccessNumber(t *testing.T) {
	result := errutil.SafeExecuteResult(func() int { return 42 })
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != 42 {
		t.Errorf("expected 42, got %v", result.Value)
	}
}

func TestSafeExecuteResultSuccessObject(t *testing.T) {
	type obj struct {
		Key string
	}
	expected := obj{Key: "value"}
	result := errutil.SafeExecuteResult(func() obj { return expected })
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value.Key != "value" {
		t.Errorf("expected Key='value', got %q", result.Value.Key)
	}
}

func TestSafeExecuteResultPanicWithError(t *testing.T) {
	wrapped := errors.New("test error")
	result := errutil.SafeExecuteResult(func() int { panic(wrapped) })
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	got, ok := result.Error.(error)
	if !ok {
		t.Fatalf("expected error type, got %T", result.Error)
	}
	if got.Error() != "test error" {
		t.Errorf("expected 'test error', got %q", got.Error())
	}
}

func TestSafeExecuteResultPanicWithString(t *testing.T) {
	result := errutil.SafeExecuteResult(func() int { panic("string error") })
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	if result.Error != "string error" {
		t.Errorf("expected 'string error', got %v", result.Error)
	}
}

func TestSafeExecuteResultPanicWithCustomType(t *testing.T) {
	result := errutil.SafeExecuteResult(func() int { panic(42) })
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	if result.Error != 42 {
		t.Errorf("expected 42, got %v", result.Error)
	}
}

// ---------------------------------------------------------------------------
// MapResult tests
// ---------------------------------------------------------------------------

func TestMapResultTransformsSuccessValue(t *testing.T) {
	success := errutil.SuccessFunction[int, error](5)
	result := errutil.MapResult(success, func(n int) int { return n * 2 })
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != 10 {
		t.Errorf("expected 10, got %v", result.Value)
	}
}

func TestMapResultReturnsErrorUnchanged(t *testing.T) {
	failure := errutil.ErrorFunction[int, string]("something went wrong")
	result := errutil.MapResult(failure, func(n int) int { return n * 2 })
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	if result.Error != "something went wrong" {
		t.Errorf("expected 'something went wrong', got %v", result.Error)
	}
}

func TestMapResultHandlesTypeTransformation(t *testing.T) {
	success := errutil.SuccessFunction[int, error](42)
	result := errutil.MapResult(success, func(n int) string {
		return fmt.Sprintf("%d", n)
	})
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != "42" {
		t.Errorf("expected '42', got %v", result.Value)
	}
}

func TestMapResultHandlesIdentityMapping(t *testing.T) {
	success := errutil.SuccessFunction[string, error]("hello")
	result := errutil.MapResult(success, func(v string) string { return v })
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != "hello" {
		t.Errorf("expected 'hello', got %v", result.Value)
	}
}

// ---------------------------------------------------------------------------
// FlatMapResult tests
// ---------------------------------------------------------------------------

func TestFlatMapResultChainsSuccess(t *testing.T) {
	success := errutil.SuccessFunction[int, string](5)
	result := errutil.FlatMapResult(success, func(n int) errutil.Result[int, string] {
		return errutil.SuccessFunction[int, string](n * 2)
	})
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != 10 {
		t.Errorf("expected 10, got %v", result.Value)
	}
}

func TestFlatMapResultReturnsOriginalError(t *testing.T) {
	failure := errutil.ErrorFunction[int, string]("original error")
	result := errutil.FlatMapResult(failure, func(n int) errutil.Result[int, string] {
		return errutil.SuccessFunction[int, string](n * 2)
	})
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	if result.Error != "original error" {
		t.Errorf("expected 'original error', got %v", result.Error)
	}
}

func TestFlatMapResultReturnsErrorFromMapping(t *testing.T) {
	success := errutil.SuccessFunction[int, string](-1)
	result := errutil.FlatMapResult(success, func(n int) errutil.Result[int, string] {
		if n > 0 {
			return errutil.SuccessFunction[int, string](n)
		}
		return errutil.ErrorFunction[int, string]("negative")
	})
	if result.Type != "error" {
		t.Fatalf("expected type 'error', got %q", result.Type)
	}
	if result.Error != "negative" {
		t.Errorf("expected 'negative', got %v", result.Error)
	}
}

func TestFlatMapResultSupportsTypeTransformation(t *testing.T) {
	success := errutil.SuccessFunction[int, string](42)
	result := errutil.FlatMapResult(success, func(n int) errutil.Result[string, string] {
		return errutil.SuccessFunction[string, string](fmt.Sprintf("%d", n))
	})
	if result.Type != "success" {
		t.Fatalf("expected type 'success', got %q", result.Type)
	}
	if result.Value != "42" {
		t.Errorf("expected '42', got %v", result.Value)
	}
}

// ---------------------------------------------------------------------------
// MatchResult tests
// ---------------------------------------------------------------------------

func TestMatchResultCallsOnSuccess(t *testing.T) {
	success := errutil.SuccessFunction[int, error](42)
	result := errutil.MatchResult(success,
		func(v int) string { return fmt.Sprintf("Got %d", v) },
		func(e error) string { return "Failed: " + e.Error() },
	)
	if result != "Got 42" {
		t.Errorf("expected 'Got 42', got %q", result)
	}
}

func TestMatchResultCallsOnError(t *testing.T) {
	failure := errutil.ErrorFunction[int](errors.New("fail"))
	result := errutil.MatchResult(failure,
		func(v int) string { return fmt.Sprintf("Got %d", v) },
		func(e error) string { return "Failed: " + e.Error() },
	)
	if result != "Failed: fail" {
		t.Errorf("expected 'Failed: fail', got %q", result)
	}
}

func TestMatchResultReturnsCorrectTypeFromOnSuccess(t *testing.T) {
	success := errutil.SuccessFunction[string, error]("hello")
	result := errutil.MatchResult(success,
		func(v string) int { return len(v) },
		func(e error) int { return -1 },
	)
	if result != 5 {
		t.Errorf("expected 5, got %d", result)
	}
}

func TestMatchResultHandlesDifferentReturnShapes(t *testing.T) {
	type matched struct {
		Found bool
		Value string
	}
	failure := errutil.ErrorFunction[int, string]("not found")
	result := errutil.MatchResult(failure,
		func(v int) matched { return matched{Found: true, Value: fmt.Sprintf("%d", v)} },
		func(e string) matched { return matched{Found: false, Value: e} },
	)
	if result.Found != false || result.Value != "not found" {
		t.Errorf("expected {false, 'not found'}, got %+v", result)
	}
}

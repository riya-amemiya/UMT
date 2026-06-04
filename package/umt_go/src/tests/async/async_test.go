package async_test

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/riya-amemiya/umt-go/src/async"
)

// ---------------------------------------------------------------------------
// Sleep tests
// ---------------------------------------------------------------------------

func TestSleepResolvesAfterDelay(t *testing.T) {
	start := time.Now()
	async.Sleep(20)
	if elapsed := time.Since(start); elapsed < 15*time.Millisecond {
		t.Errorf("expected at least ~20ms elapsed, got %v", elapsed)
	}
}

func TestSleepZeroReturnsImmediately(t *testing.T) {
	start := time.Now()
	async.Sleep(0)
	if elapsed := time.Since(start); elapsed > 10*time.Millisecond {
		t.Errorf("expected near-immediate return, got %v", elapsed)
	}
}

func TestSleepNegativeReturnsImmediately(t *testing.T) {
	start := time.Now()
	async.Sleep(-100)
	if elapsed := time.Since(start); elapsed > 10*time.Millisecond {
		t.Errorf("expected near-immediate return, got %v", elapsed)
	}
}

// ---------------------------------------------------------------------------
// Deferred tests
// ---------------------------------------------------------------------------

func TestDeferredResolvesWithValue(t *testing.T) {
	d := async.NewDeferred[int]()
	d.Resolve(42)
	value, err := d.Await()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != 42 {
		t.Errorf("expected 42, got %v", value)
	}
}

func TestDeferredRejectsWithReason(t *testing.T) {
	d := async.NewDeferred[int]()
	d.Reject(errors.New("deferred error"))
	_, err := d.Await()
	if err == nil || err.Error() != "deferred error" {
		t.Errorf("expected 'deferred error', got %v", err)
	}
}

func TestDeferredWorksWithDelayedResolve(t *testing.T) {
	d := async.NewDeferred[string]()
	go func() {
		async.Sleep(10)
		d.Resolve("delayed")
	}()
	value, err := d.Await()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != "delayed" {
		t.Errorf("expected 'delayed', got %q", value)
	}
}

func TestDeferredSettlesOnlyOnce(t *testing.T) {
	d := async.NewDeferred[int]()
	d.Resolve(1)
	d.Resolve(2)
	d.Reject(errors.New("ignored"))
	value, err := d.Await()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != 1 {
		t.Errorf("expected first settlement 1, got %v", value)
	}
}

// ---------------------------------------------------------------------------
// Timeout tests
// ---------------------------------------------------------------------------

func TestTimeoutResolvesBeforeDeadline(t *testing.T) {
	value, err := async.Timeout(func() (string, error) {
		async.Sleep(10)
		return "done", nil
	}, 1000)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != "done" {
		t.Errorf("expected 'done', got %q", value)
	}
}

func TestTimeoutRejectsWhenExceeded(t *testing.T) {
	_, err := async.Timeout(func() (string, error) {
		async.Sleep(200)
		return "done", nil
	}, 30)
	if err == nil || err.Error() != "Timed out after 30ms" {
		t.Errorf("expected 'Timed out after 30ms', got %v", err)
	}
}

func TestTimeoutPropagatesOriginalError(t *testing.T) {
	_, err := async.Timeout(func() (string, error) {
		async.Sleep(5)
		return "", errors.New("original error")
	}, 1000)
	if err == nil || err.Error() != "original error" {
		t.Errorf("expected 'original error', got %v", err)
	}
}

// ---------------------------------------------------------------------------
// Parallel tests
// ---------------------------------------------------------------------------

func TestParallelProcessesInOrder(t *testing.T) {
	items := []int{1, 2, 3, 4, 5}
	results, err := async.Parallel(2, items, func(n, _ int) (int, error) {
		return n * 10, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := []int{10, 20, 30, 40, 50}
	for i, v := range expected {
		if results[i] != v {
			t.Errorf("at %d expected %d, got %d", i, v, results[i])
		}
	}
}

func TestParallelRespectsConcurrencyLimit(t *testing.T) {
	var running int32
	var maxRunning int32
	items := []int{1, 2, 3, 4, 5, 6}
	_, err := async.Parallel(3, items, func(n, _ int) (int, error) {
		cur := atomic.AddInt32(&running, 1)
		for {
			prev := atomic.LoadInt32(&maxRunning)
			if cur <= prev || atomic.CompareAndSwapInt32(&maxRunning, prev, cur) {
				break
			}
		}
		async.Sleep(10)
		atomic.AddInt32(&running, -1)
		return n, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if maxRunning > 3 {
		t.Errorf("expected max concurrency <= 3, got %d", maxRunning)
	}
}

func TestParallelEmpty(t *testing.T) {
	results, err := async.Parallel(2, []int{}, func(n, _ int) (int, error) {
		return n * 2, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(results) != 0 {
		t.Errorf("expected empty results, got %v", results)
	}
}

func TestParallelLimitLargerThanItems(t *testing.T) {
	results, err := async.Parallel(10, []int{1, 2}, func(n, _ int) (int, error) {
		return n + 1, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(results) != 2 || results[0] != 2 || results[1] != 3 {
		t.Errorf("expected [2 3], got %v", results)
	}
}

func TestParallelRejectsOnFailure(t *testing.T) {
	_, err := async.Parallel(2, []int{1, 2, 3}, func(n, _ int) (int, error) {
		if n == 2 {
			return 0, errors.New("fail")
		}
		return n, nil
	})
	if err == nil || err.Error() != "fail" {
		t.Errorf("expected 'fail', got %v", err)
	}
}

func TestParallelProvidesIndex(t *testing.T) {
	items := []string{"a", "b", "c"}
	results, err := async.Parallel(2, items, func(item string, index int) (string, error) {
		return fmt.Sprintf("%s-%d", item, index), nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	expected := []string{"a-0", "b-1", "c-2"}
	for i, v := range expected {
		if results[i] != v {
			t.Errorf("at %d expected %q, got %q", i, v, results[i])
		}
	}
}

func TestParallelSequentialWithLimitOne(t *testing.T) {
	var mu sync.Mutex
	order := []int{}
	items := []int{1, 2, 3}
	_, err := async.Parallel(1, items, func(n, _ int) (int, error) {
		mu.Lock()
		order = append(order, n)
		mu.Unlock()
		return n, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(order) != 3 || order[0] != 1 || order[1] != 2 || order[2] != 3 {
		t.Errorf("expected [1 2 3], got %v", order)
	}
}

// ---------------------------------------------------------------------------
// PSettled tests
// ---------------------------------------------------------------------------

func TestPSettledMixedResults(t *testing.T) {
	results := async.PSettled([]func() (int, error){
		func() (int, error) { return 1, nil },
		func() (int, error) { return 0, errors.New("nope") },
	})
	if results[0].Status != "fulfilled" || results[0].Value != 1 {
		t.Errorf("expected fulfilled 1, got %+v", results[0])
	}
	if results[1].Status != "rejected" || results[1].Reason == nil {
		t.Errorf("expected rejected, got %+v", results[1])
	}
}

func TestPSettledEmpty(t *testing.T) {
	results := async.PSettled([]func() (int, error){})
	if len(results) != 0 {
		t.Errorf("expected empty results, got %v", results)
	}
}

func TestPSettledRunsAllTasks(t *testing.T) {
	var started int32
	results := async.PSettled([]func() (int, error){
		func() (int, error) { atomic.AddInt32(&started, 1); return 1, nil },
		func() (int, error) { atomic.AddInt32(&started, 1); return 2, nil },
	}, 1)
	if started != 2 {
		t.Errorf("expected 2 started, got %d", started)
	}
	if results[0].Value != 1 || results[1].Value != 2 {
		t.Errorf("expected [1 2], got %+v", results)
	}
}

func TestPSettledRespectsConcurrencyLimit(t *testing.T) {
	var active int32
	var peak int32
	tasks := make([]func() (int, error), 6)
	for i := range tasks {
		index := i
		tasks[i] = func() (int, error) {
			cur := atomic.AddInt32(&active, 1)
			for {
				prev := atomic.LoadInt32(&peak)
				if cur <= prev || atomic.CompareAndSwapInt32(&peak, prev, cur) {
					break
				}
			}
			async.Sleep(5)
			atomic.AddInt32(&active, -1)
			return index, nil
		}
	}
	async.PSettled(tasks, 2)
	if peak > 2 {
		t.Errorf("expected peak concurrency <= 2, got %d", peak)
	}
}

func TestPSettledPreservesOrder(t *testing.T) {
	results := async.PSettled([]func() (int, error){
		func() (int, error) { async.Sleep(20); return 1, nil },
		func() (int, error) { return 2, nil },
	})
	if results[0].Value != 1 || results[1].Value != 2 {
		t.Errorf("expected [1 2], got %+v", results)
	}
}

func TestPSettledTreatsZeroLimitAsUnlimited(t *testing.T) {
	results := async.PSettled([]func() (int, error){
		func() (int, error) { return 1, nil },
		func() (int, error) { return 2, nil },
	}, 0)
	if len(results) != 2 {
		t.Errorf("expected length 2, got %d", len(results))
	}
}

// ---------------------------------------------------------------------------
// Retry tests
// ---------------------------------------------------------------------------

func TestRetryFirstSuccess(t *testing.T) {
	calls := 0
	value, err := async.Retry(func() (string, error) {
		calls++
		return "ok", nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != "ok" || calls != 1 {
		t.Errorf("expected ok/1 call, got %q/%d", value, calls)
	}
}

func TestRetryEventuallyResolves(t *testing.T) {
	attempts := 0
	value, err := async.Retry(func() (string, error) {
		attempts++
		if attempts < 3 {
			return "", errors.New("nope")
		}
		return "ok", nil
	}, async.RetryOptions{}.WithRetries(3).WithDelay(1))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != "ok" || attempts != 3 {
		t.Errorf("expected ok/3 attempts, got %q/%d", value, attempts)
	}
}

func TestRetryExhausts(t *testing.T) {
	calls := 0
	_, err := async.Retry(func() (string, error) {
		calls++
		return "", errors.New("nope")
	}, async.RetryOptions{}.WithRetries(2).WithDelay(1))
	if err == nil || err.Error() != "nope" {
		t.Errorf("expected 'nope', got %v", err)
	}
	if calls != 3 {
		t.Errorf("expected 3 calls, got %d", calls)
	}
}

func TestRetryShouldRetryFalse(t *testing.T) {
	calls := 0
	_, err := async.Retry(func() (string, error) {
		calls++
		return "", errors.New("fatal")
	}, async.RetryOptions{
		ShouldRetry: func(error, int) bool { return false },
	}.WithRetries(5).WithDelay(1))
	if err == nil || err.Error() != "fatal" {
		t.Errorf("expected 'fatal', got %v", err)
	}
	if calls != 1 {
		t.Errorf("expected 1 call, got %d", calls)
	}
}

func TestRetryOnRetryCallback(t *testing.T) {
	attempts := 0
	onRetryCalls := 0
	_, err := async.Retry(func() (string, error) {
		attempts++
		if attempts < 2 {
			return "", errors.New("nope")
		}
		return "ok", nil
	}, async.RetryOptions{
		OnRetry: func(error, int) { onRetryCalls++ },
	}.WithRetries(2).WithDelay(1))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if onRetryCalls != 1 {
		t.Errorf("expected 1 onRetry call, got %d", onRetryCalls)
	}
}

func TestRetryLinearBackoff(t *testing.T) {
	attempts := 0
	_, err := async.Retry(func() (string, error) {
		attempts++
		if attempts < 2 {
			return "", errors.New("nope")
		}
		return "ok", nil
	}, async.RetryOptions{Backoff: "linear"}.WithRetries(2).WithDelay(1))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if attempts != 2 {
		t.Errorf("expected 2 attempts, got %d", attempts)
	}
}

func TestRetryExponentialBackoff(t *testing.T) {
	attempts := 0
	_, err := async.Retry(func() (string, error) {
		attempts++
		if attempts < 2 {
			return "", errors.New("nope")
		}
		return "ok", nil
	}, async.RetryOptions{Backoff: "exponential"}.WithRetries(2).WithDelay(1))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if attempts != 2 {
		t.Errorf("expected 2 attempts, got %d", attempts)
	}
}

func TestRetryJitter(t *testing.T) {
	attempts := 0
	_, err := async.Retry(func() (string, error) {
		attempts++
		if attempts < 2 {
			return "", errors.New("nope")
		}
		return "ok", nil
	}, async.RetryOptions{Jitter: true}.WithRetries(2).WithDelay(1))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if attempts != 2 {
		t.Errorf("expected 2 attempts, got %d", attempts)
	}
}

func TestRetryRejectsWhenAlreadyAborted(t *testing.T) {
	ctx, cancel := context.WithCancelCause(context.Background())
	cancel(errors.New("cancelled"))
	_, err := async.Retry(func() (string, error) {
		return "ok", nil
	}, async.RetryOptions{Ctx: ctx})
	if err == nil || err.Error() != "cancelled" {
		t.Errorf("expected 'cancelled', got %v", err)
	}
}

func TestRetryGenericAbortedWhenNoCause(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err := async.Retry(func() (string, error) {
		return "ok", nil
	}, async.RetryOptions{Ctx: ctx})
	if err == nil || err.Error() != "Aborted" {
		t.Errorf("expected 'Aborted', got %v", err)
	}
}

// ---------------------------------------------------------------------------
// WaitFor tests
// ---------------------------------------------------------------------------

func TestWaitForImmediatelyReady(t *testing.T) {
	value, err := async.WaitFor(func() (int, bool, error) {
		return 42, true, nil
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != 42 {
		t.Errorf("expected 42, got %d", value)
	}
}

func TestWaitForBecomesReady(t *testing.T) {
	counter := 0
	value, err := async.WaitFor(func() (string, bool, error) {
		counter++
		if counter >= 3 {
			return "done", true, nil
		}
		return "", false, nil
	}, async.WaitForOptions{Interval: 1, Timeout: 500})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if value != "done" {
		t.Errorf("expected 'done', got %q", value)
	}
}

func TestWaitForTimesOut(t *testing.T) {
	_, err := async.WaitFor(func() (int, bool, error) {
		return 0, false, nil
	}, async.WaitForOptions{Interval: 5, Timeout: 20})
	if err == nil || err.Error() != "waitFor timed out after 20ms" {
		t.Errorf("expected timeout error, got %v", err)
	}
}

func TestWaitForRejectsWhenConditionErrors(t *testing.T) {
	_, err := async.WaitFor(func() (int, bool, error) {
		return 0, false, errors.New("boom")
	}, async.WaitForOptions{Timeout: 100})
	if err == nil || err.Error() != "boom" {
		t.Errorf("expected 'boom', got %v", err)
	}
}

func TestWaitForRejectsWhenAlreadyAborted(t *testing.T) {
	ctx, cancel := context.WithCancelCause(context.Background())
	cancel(errors.New("nope"))
	_, err := async.WaitFor(func() (int, bool, error) {
		return 1, true, nil
	}, async.WaitForOptions{Ctx: ctx})
	if err == nil || err.Error() != "nope" {
		t.Errorf("expected 'nope', got %v", err)
	}
}

func TestWaitForAbortsDuringPolling(t *testing.T) {
	ctx, cancel := context.WithCancelCause(context.Background())
	go func() {
		async.Sleep(5)
		cancel(errors.New("stopped"))
	}()
	_, err := async.WaitFor(func() (int, bool, error) {
		return 0, false, nil
	}, async.WaitForOptions{Ctx: ctx, Interval: 2, Timeout: 1000})
	if err == nil || err.Error() != "stopped" {
		t.Errorf("expected 'stopped', got %v", err)
	}
}

func TestWaitForGenericAbortedWhenNoCause(t *testing.T) {
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, err := async.WaitFor(func() (int, bool, error) {
		return 0, false, nil
	}, async.WaitForOptions{Ctx: ctx})
	if err == nil || err.Error() != "Aborted" {
		t.Errorf("expected 'Aborted', got %v", err)
	}
}

// ---------------------------------------------------------------------------
// DebounceAsync tests
// ---------------------------------------------------------------------------

func TestDebounceAsyncCoalescesRapidCalls(t *testing.T) {
	var calls int32
	var lastArg int32
	debounced := async.DebounceAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		atomic.StoreInt32(&lastArg, int32(value))
		return value * 2, nil
	}, 10)

	d1 := debounced.Call(1)
	d2 := debounced.Call(2)
	d3 := debounced.Call(3)

	r1, _ := d1.Await()
	r2, _ := d2.Await()
	r3, _ := d3.Await()

	if calls != 1 {
		t.Errorf("expected 1 call, got %d", calls)
	}
	if lastArg != 3 {
		t.Errorf("expected last arg 3, got %d", lastArg)
	}
	if r1 != 6 || r2 != 6 || r3 != 6 {
		t.Errorf("expected all 6, got %d %d %d", r1, r2, r3)
	}
}

func TestDebounceAsyncInvokesAgainAfterWait(t *testing.T) {
	var calls int32
	debounced := async.DebounceAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		return value, nil
	}, 5)

	debounced.Call(1).Await()
	async.Sleep(10)
	debounced.Call(2).Await()

	if calls != 2 {
		t.Errorf("expected 2 calls, got %d", calls)
	}
}

func TestDebounceAsyncCancelRejectsPending(t *testing.T) {
	debounced := async.DebounceAsync(func(value int) (int, error) {
		return value, nil
	}, 50)
	pending := debounced.Call(1)
	debounced.Cancel()
	_, err := pending.Await()
	if err == nil || err.Error() != "debounceAsync cancelled" {
		t.Errorf("expected 'debounceAsync cancelled', got %v", err)
	}
}

func TestDebounceAsyncPropagatesRejection(t *testing.T) {
	debounced := async.DebounceAsync(func(int) (int, error) {
		return 0, errors.New("boom")
	}, 5)
	_, err := debounced.Call(0).Await()
	if err == nil || err.Error() != "boom" {
		t.Errorf("expected 'boom', got %v", err)
	}
}

func TestDebounceAsyncCancelNoOpWhenIdle(t *testing.T) {
	debounced := async.DebounceAsync(func(int) (int, error) {
		return 1, nil
	}, 10)
	debounced.Cancel()
}

// ---------------------------------------------------------------------------
// ThrottleAsync tests
// ---------------------------------------------------------------------------

func TestThrottleAsyncSharesInflight(t *testing.T) {
	var calls int32
	throttled := async.ThrottleAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		async.Sleep(10)
		return value, nil
	}, 0)
	a := throttled.Call(1)
	b := throttled.Call(2)
	if a != b {
		t.Error("expected concurrent callers to receive the same future")
	}
	a.Await()
	b.Await()
	if calls != 1 {
		t.Errorf("expected 1 call, got %d", calls)
	}
}

func TestThrottleAsyncRejectsWithinLockout(t *testing.T) {
	var calls int32
	throttled := async.ThrottleAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		return value, nil
	}, 1000)
	throttled.Call(1).Await()
	_, err := throttled.Call(2).Await()
	if err == nil || err.Error() != "throttleAsync window not elapsed" {
		t.Errorf("expected lockout error, got %v", err)
	}
	if calls != 1 {
		t.Errorf("expected 1 call, got %d", calls)
	}
}

func TestThrottleAsyncAllowsAfterWindow(t *testing.T) {
	var calls int32
	throttled := async.ThrottleAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		return value, nil
	}, 5)
	throttled.Call(1).Await()
	async.Sleep(15)
	throttled.Call(2).Await()
	if calls != 2 {
		t.Errorf("expected 2 calls, got %d", calls)
	}
}

func TestThrottleAsyncCancelClearsLockout(t *testing.T) {
	var calls int32
	throttled := async.ThrottleAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		return value, nil
	}, 1000)
	throttled.Call(1).Await()
	throttled.Cancel()
	throttled.Call(2).Await()
	if calls != 2 {
		t.Errorf("expected 2 calls, got %d", calls)
	}
}

func TestThrottleAsyncPropagatesRejection(t *testing.T) {
	throttled := async.ThrottleAsync(func(int) (int, error) {
		return 0, errors.New("boom")
	}, 5)
	_, err := throttled.Call(0).Await()
	if err == nil || err.Error() != "boom" {
		t.Errorf("expected 'boom', got %v", err)
	}
}

func TestThrottleAsyncOlderResolveDoesNotClearNewer(t *testing.T) {
	var calls int32
	resolveFirst := make(chan int)
	resolveSecond := make(chan int)
	throttled := async.ThrottleAsync(func(value int) (int, error) {
		atomic.AddInt32(&calls, 1)
		if value == 1 {
			return <-resolveFirst, nil
		}
		return <-resolveSecond, nil
	}, 1000)

	first := throttled.Call(1)
	throttled.Cancel()
	second := throttled.Call(2)
	resolveFirst <- 1
	first.Await()
	resolveSecond <- 2
	second.Await()
	if calls != 2 {
		t.Errorf("expected 2 calls, got %d", calls)
	}
}

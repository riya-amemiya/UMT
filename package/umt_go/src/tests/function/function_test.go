package function_test

import (
	"fmt"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/riya-amemiya/umt-go/src/function"
)

// ---------------------------------------------------------------------------
// Curry tests
// ---------------------------------------------------------------------------

func TestCurryZeroArguments(t *testing.T) {
	fn := func(args ...any) any { return "hello" }
	curried := function.Curry(fn, 0)
	if got := curried.Value; got != "hello" {
		t.Errorf("expected hello, got %v", got)
	}
	if !curried.Done {
		t.Errorf("expected Done true for zero-arity curry")
	}
}

func TestCurryOneArgument(t *testing.T) {
	fn := func(args ...any) any { return args[0].(int) * 2 }
	curried := function.Curry(fn, 1)
	if got := curried.Call(5).Value; got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
}

func TestCurryTwoArguments(t *testing.T) {
	fn := func(args ...any) any { return args[0].(int) + args[1].(int) }
	curried := function.Curry(fn, 2)
	if got := curried.Call(2).Call(3).Value; got != 5 {
		t.Errorf("expected 5 from chained call, got %v", got)
	}
	if got := curried.Call(2, 3).Value; got != 5 {
		t.Errorf("expected 5 from single call, got %v", got)
	}
}

func TestCurryThreeArguments(t *testing.T) {
	fn := func(args ...any) any { return args[0].(int)*args[1].(int) + args[2].(int) }
	curried := function.Curry(fn, 3)
	cases := []*function.CurriedFunc{
		curried.Call(2).Call(3).Call(4),
		curried.Call(2, 3).Call(4),
		curried.Call(2).Call(3, 4),
		curried.Call(2, 3, 4),
	}
	for i, c := range cases {
		if c.Value != 10 {
			t.Errorf("case %d: expected 10, got %v", i, c.Value)
		}
	}
}

func TestCurryFourArguments(t *testing.T) {
	fn := func(args ...any) any {
		return args[0].(int) + args[1].(int) + args[2].(int) + args[3].(int)
	}
	curried := function.Curry(fn, 4)
	cases := []*function.CurriedFunc{
		curried.Call(1).Call(2).Call(3).Call(4),
		curried.Call(1, 2).Call(3).Call(4),
		curried.Call(1).Call(2, 3).Call(4),
		curried.Call(1).Call(2).Call(3, 4),
		curried.Call(1, 2, 3).Call(4),
		curried.Call(1, 2).Call(3, 4),
		curried.Call(1, 2, 3, 4),
	}
	for i, c := range cases {
		if c.Value != 10 {
			t.Errorf("case %d: expected 10, got %v", i, c.Value)
		}
	}
}

func TestCurryFiveArguments(t *testing.T) {
	fn := func(args ...any) any {
		return args[0].(int) + args[1].(int) + args[2].(int) + args[3].(int) + args[4].(int)
	}
	curried := function.Curry(fn, 5)
	cases := []*function.CurriedFunc{
		curried.Call(1).Call(2).Call(3).Call(4).Call(5),
		curried.Call(1, 2).Call(3).Call(4).Call(5),
		curried.Call(1, 2, 3).Call(4, 5),
		curried.Call(1, 2, 3, 4, 5),
	}
	for i, c := range cases {
		if c.Value != 15 {
			t.Errorf("case %d: expected 15, got %v", i, c.Value)
		}
	}
}

func TestCurrySixArguments(t *testing.T) {
	fn := func(args ...any) any {
		sum := 0
		for _, a := range args {
			sum += a.(int)
		}
		return sum
	}
	curried := function.Curry(fn, 6)
	cases := []*function.CurriedFunc{
		curried.Call(1).Call(2).Call(3).Call(4).Call(5).Call(6),
		curried.Call(1, 2, 3).Call(4, 5, 6),
		curried.Call(1, 2, 3, 4, 5, 6),
	}
	for i, c := range cases {
		if c.Value != 21 {
			t.Errorf("case %d: expected 21, got %v", i, c.Value)
		}
	}
}

func TestCurryDifferentArgumentTypes(t *testing.T) {
	fn := func(args ...any) any {
		return fmt.Sprintf("%d%s%t", args[0].(int), args[1].(string), args[2].(bool))
	}
	curried := function.Curry(fn, 3)
	if got := curried.Call(1).Call("hello").Call(true).Value; got != "1hellotrue" {
		t.Errorf("expected 1hellotrue, got %v", got)
	}
	if got := curried.Call(1, "hello", true).Value; got != "1hellotrue" {
		t.Errorf("expected 1hellotrue, got %v", got)
	}
}

func TestCurryObjectAndSliceArguments(t *testing.T) {
	fn := func(args ...any) any {
		x := args[0].(map[string]int)["x"]
		sum := 0
		for _, v := range args[1].([]int) {
			sum += v
		}
		return x + sum
	}
	curried := function.Curry(fn, 2)
	if got := curried.Call(map[string]int{"x": 5}).Call([]int{1, 2, 3}).Value; got != 11 {
		t.Errorf("expected 11, got %v", got)
	}
	if got := curried.Call(map[string]int{"x": 5}, []int{1, 2, 3}).Value; got != 11 {
		t.Errorf("expected 11, got %v", got)
	}
}

func TestCurryFunctionArguments(t *testing.T) {
	fn := func(args ...any) any {
		f := args[0].(func(int) int)
		return f(args[1].(int))
	}
	curried := function.Curry(fn, 2)
	double := func(x int) int { return x * 2 }
	if got := curried.Call(double).Call(5).Value; got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
	if got := curried.Call(double, 5).Value; got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
}

// ---------------------------------------------------------------------------
// Once tests
// ---------------------------------------------------------------------------

func TestOnceInvokesOnlyOnce(t *testing.T) {
	var calls int
	fn := function.Once(func(args ...any) int {
		calls++
		return 42
	})
	if got := fn(); got != 42 {
		t.Errorf("expected 42, got %v", got)
	}
	if got := fn(); got != 42 {
		t.Errorf("expected 42, got %v", got)
	}
	if got := fn(); got != 42 {
		t.Errorf("expected 42, got %v", got)
	}
	if calls != 1 {
		t.Errorf("expected fn called once, got %d", calls)
	}
}

func TestOncePassesFirstArguments(t *testing.T) {
	var captured []any
	fn := function.Once(func(args ...any) int {
		captured = args
		return args[0].(int) + args[1].(int)
	})
	if got := fn(3, 4); got != 7 {
		t.Errorf("expected 7, got %v", got)
	}
	if got := fn(10, 20); got != 7 {
		t.Errorf("expected first result 7, got %v", got)
	}
	if captured[0] != 3 || captured[1] != 4 {
		t.Errorf("expected first args (3, 4), got %v", captured)
	}
}

func TestOnceReturnsFirstResultDespiteDifferentArguments(t *testing.T) {
	fn := function.Once(func(args ...any) string {
		s := args[0].(string)
		out := ""
		for _, r := range s {
			if r >= 'a' && r <= 'z' {
				out += string(r - 32)
			} else {
				out += string(r)
			}
		}
		return out
	})
	if got := fn("hello"); got != "HELLO" {
		t.Errorf("expected HELLO, got %v", got)
	}
	if got := fn("world"); got != "HELLO" {
		t.Errorf("expected HELLO, got %v", got)
	}
}

func TestOnceConcurrentRunsOnce(t *testing.T) {
	var calls int32
	fn := function.Once(func(args ...any) int {
		atomic.AddInt32(&calls, 1)
		return 1
	})
	var wg sync.WaitGroup
	for i := 0; i < 50; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			fn()
		}()
	}
	wg.Wait()
	if calls != 1 {
		t.Errorf("expected fn called once under concurrency, got %d", calls)
	}
}

// ---------------------------------------------------------------------------
// Memoize tests
// ---------------------------------------------------------------------------

func TestMemoizeCachesSameArgument(t *testing.T) {
	var calls int
	memoized := function.Memoize[int, int](func(args ...any) int {
		calls++
		return args[0].(int) * 2
	})
	if got := memoized.Call(5); got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
	if got := memoized.Call(5); got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
	if calls != 1 {
		t.Errorf("expected 1 computation, got %d", calls)
	}
}

func TestMemoizeComputesForDifferentArguments(t *testing.T) {
	var calls int
	memoized := function.Memoize[int, int](func(args ...any) int {
		calls++
		return args[0].(int) * 2
	})
	if got := memoized.Call(5); got != 10 {
		t.Errorf("expected 10, got %v", got)
	}
	if got := memoized.Call(10); got != 20 {
		t.Errorf("expected 20, got %v", got)
	}
	if calls != 2 {
		t.Errorf("expected 2 computations, got %d", calls)
	}
}

func TestMemoizeExposesCache(t *testing.T) {
	memoized := function.Memoize[int, int](func(args ...any) int {
		return args[0].(int) * 2
	})
	memoized.Call(5)
	if got := memoized.Cache.Size(); got != 1 {
		t.Errorf("expected cache size 1, got %d", got)
	}
	if value, ok := memoized.Cache.Get(5); !ok || value != 10 {
		t.Errorf("expected cache[5] == 10, got %v (ok=%t)", value, ok)
	}
}

func TestMemoizeEvictsOldestWhenMaxSizeExceeded(t *testing.T) {
	var calls int
	memoized := function.Memoize[int, int](
		func(args ...any) int {
			calls++
			return args[0].(int) * 2
		},
		function.WithMaxSize[int, int](2),
	)
	memoized.Call(1)
	memoized.Call(2)
	memoized.Call(3)
	if got := memoized.Cache.Size(); got != 2 {
		t.Errorf("expected cache size 2, got %d", got)
	}
	if memoized.Cache.Has(1) {
		t.Errorf("expected key 1 evicted")
	}
	if !memoized.Cache.Has(2) {
		t.Errorf("expected key 2 retained")
	}
	if !memoized.Cache.Has(3) {
		t.Errorf("expected key 3 retained")
	}
}

func TestMemoizeUsesCustomResolver(t *testing.T) {
	var calls int
	memoized := function.Memoize[string, int](
		func(args ...any) int {
			calls++
			return args[0].(int) + args[1].(int)
		},
		function.WithResolver[string, int](func(args ...any) string {
			return fmt.Sprintf("%v-%v", args[0], args[1])
		}),
	)
	if got := memoized.Call(1, 2); got != 3 {
		t.Errorf("expected 3, got %v", got)
	}
	if got := memoized.Call(1, 2); got != 3 {
		t.Errorf("expected 3, got %v", got)
	}
	if calls != 1 {
		t.Errorf("expected 1 computation, got %d", calls)
	}
	if got := memoized.Call(2, 1); got != 3 {
		t.Errorf("expected 3, got %v", got)
	}
	if calls != 2 {
		t.Errorf("expected 2 computations, got %d", calls)
	}
}

func TestMemoizeStringKeysByDefault(t *testing.T) {
	var calls int
	memoized := function.Memoize[string, string](func(args ...any) string {
		calls++
		s := args[0].(string)
		out := ""
		for _, r := range s {
			if r >= 'a' && r <= 'z' {
				out += string(r - 32)
			} else {
				out += string(r)
			}
		}
		return out
	})
	if got := memoized.Call("hello"); got != "HELLO" {
		t.Errorf("expected HELLO, got %v", got)
	}
	if got := memoized.Call("hello"); got != "HELLO" {
		t.Errorf("expected HELLO, got %v", got)
	}
	if calls != 1 {
		t.Errorf("expected 1 computation, got %d", calls)
	}
}

func TestMemoizeEvictsOldestWithResolverAndMaxSize(t *testing.T) {
	memoized := function.Memoize[string, int](
		func(args ...any) int {
			return args[0].(int) + args[1].(int)
		},
		function.WithMaxSize[string, int](2),
		function.WithResolver[string, int](func(args ...any) string {
			return fmt.Sprintf("%v-%v", args[0], args[1])
		}),
	)
	memoized.Call(1, 2)
	memoized.Call(3, 4)
	memoized.Call(5, 6)
	if got := memoized.Cache.Size(); got != 2 {
		t.Errorf("expected cache size 2, got %d", got)
	}
	if memoized.Cache.Has("1-2") {
		t.Errorf("expected key 1-2 evicted")
	}
	if !memoized.Cache.Has("3-4") {
		t.Errorf("expected key 3-4 retained")
	}
	if !memoized.Cache.Has("5-6") {
		t.Errorf("expected key 5-6 retained")
	}
}

// ---------------------------------------------------------------------------
// Debounce tests
// ---------------------------------------------------------------------------

func TestDebounceDelaysInvocation(t *testing.T) {
	var calls int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	debounced.Call()
	if atomic.LoadInt32(&calls) != 0 {
		t.Errorf("expected no call before wait elapses")
	}
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 call after wait, got %d", got)
	}
}

func TestDebounceResetsTimerOnSubsequentCalls(t *testing.T) {
	var calls int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	debounced.Call()
	time.Sleep(50 * time.Millisecond)
	debounced.Call()
	time.Sleep(50 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 0 {
		t.Errorf("expected no call before reset window elapses, got %d", got)
	}
	time.Sleep(80 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 call after reset window, got %d", got)
	}
}

func TestDebouncePassesLatestArguments(t *testing.T) {
	var last atomic.Int32
	debounced := function.Debounce(func(args ...any) {
		last.Store(int32(args[0].(int)))
	}, 100)

	debounced.Call(1)
	debounced.Call(2)
	debounced.Call(3)
	time.Sleep(160 * time.Millisecond)
	if got := last.Load(); got != 3 {
		t.Errorf("expected latest argument 3, got %d", got)
	}
}

func TestDebounceLeadingEdge(t *testing.T) {
	var calls int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100, function.WithLeading(true), function.WithTrailing(false))

	debounced.Call()
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 leading call, got %d", got)
	}
	debounced.Call()
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected still 1 call within window, got %d", got)
	}
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected no trailing call, got %d", got)
	}
}

func TestDebounceLeadingAndTrailingEdges(t *testing.T) {
	var calls int32
	var last atomic.Int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
		last.Store(int32(args[0].(int)))
	}, 100, function.WithLeading(true), function.WithTrailing(true))

	debounced.Call(1)
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 leading call, got %d", got)
	}
	if got := last.Load(); got != 1 {
		t.Errorf("expected leading argument 1, got %d", got)
	}
	debounced.Call(2)
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 2 {
		t.Errorf("expected 2 calls (leading + trailing), got %d", got)
	}
	if got := last.Load(); got != 2 {
		t.Errorf("expected trailing argument 2, got %d", got)
	}
}

func TestDebounceCancel(t *testing.T) {
	var calls int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	debounced.Call()
	debounced.Cancel()
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 0 {
		t.Errorf("expected cancelled invocation not to run, got %d", got)
	}
}

func TestDebounceCancelWithoutPending(t *testing.T) {
	var calls int32
	debounced := function.Debounce(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	debounced.Cancel()
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 0 {
		t.Errorf("expected no call, got %d", got)
	}
}

// ---------------------------------------------------------------------------
// Throttle tests
// ---------------------------------------------------------------------------

func TestThrottleInvokesImmediately(t *testing.T) {
	var calls int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	throttled.Call()
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 immediate call, got %d", got)
	}
}

func TestThrottleThrottlesWithinWindow(t *testing.T) {
	var calls int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	throttled.Call()
	throttled.Call()
	throttled.Call()
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected 1 call within window, got %d", got)
	}
}

func TestThrottleTrailingCallAfterWindow(t *testing.T) {
	var calls int32
	var last atomic.Int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
		last.Store(int32(args[0].(int)))
	}, 100)

	throttled.Call(1)
	throttled.Call(2)
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 2 {
		t.Errorf("expected 2 calls (leading + trailing), got %d", got)
	}
	if got := last.Load(); got != 2 {
		t.Errorf("expected trailing argument 2, got %d", got)
	}
}

func TestThrottlePassesLatestTrailingArguments(t *testing.T) {
	var last atomic.Value
	throttled := function.Throttle(func(args ...any) {
		last.Store(args[0].(string))
	}, 100)

	throttled.Call("a")
	throttled.Call("b")
	throttled.Call("c")
	time.Sleep(160 * time.Millisecond)
	if got := last.Load(); got != "c" {
		t.Errorf("expected trailing argument c, got %v", got)
	}
}

func TestThrottleAllowsInvocationAfterWindow(t *testing.T) {
	var calls int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	throttled.Call()
	time.Sleep(160 * time.Millisecond)
	throttled.Call()
	if got := atomic.LoadInt32(&calls); got != 2 {
		t.Errorf("expected 2 calls across windows, got %d", got)
	}
}

func TestThrottleCancel(t *testing.T) {
	var calls int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	throttled.Call()
	throttled.Call()
	throttled.Cancel()
	time.Sleep(160 * time.Millisecond)
	if got := atomic.LoadInt32(&calls); got != 1 {
		t.Errorf("expected only the leading call to run, got %d", got)
	}
}

func TestThrottleCancelWithoutPending(t *testing.T) {
	var calls int32
	throttled := function.Throttle(func(args ...any) {
		atomic.AddInt32(&calls, 1)
	}, 100)

	throttled.Cancel()
	if got := atomic.LoadInt32(&calls); got != 0 {
		t.Errorf("expected no call, got %d", got)
	}
}

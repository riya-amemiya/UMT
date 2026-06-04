package async

import "sync"

// Parallel applies fn to every item with at most limit concurrent invocations
// and returns the results in the same order as the input items. It mirrors the
// TypeScript parallel utility: execution is fail-fast, so the first non-nil
// error returned by any invocation becomes the overall error and the remaining
// results are whatever had completed by then.
//
// An empty items slice resolves immediately with an empty result slice. A limit
// of zero or less is treated as no concurrency, in which case no work runs and
// an empty (length-matched) result slice is returned.
//
// Example:
//
//	results, err := Parallel(2, []int{1, 2, 3}, func(n, _ int) (int, error) {
//	    return n * 2, nil
//	})
//	// results == []int{2, 4, 6}
func Parallel[T, U any](limit int, items []T, fn func(item T, index int) (U, error)) ([]U, error) {
	results := make([]U, len(items))
	if len(items) == 0 {
		return results, nil
	}
	if limit <= 0 {
		return results, nil
	}

	if limit > len(items) {
		limit = len(items)
	}

	var (
		mu       sync.Mutex
		firstErr error
		wg       sync.WaitGroup
	)
	indexCh := make(chan int)

	worker := func() {
		defer wg.Done()
		for index := range indexCh {
			result, err := fn(items[index], index)
			mu.Lock()
			if err != nil {
				if firstErr == nil {
					firstErr = err
				}
			} else {
				results[index] = result
			}
			mu.Unlock()
		}
	}

	wg.Add(limit)
	for i := 0; i < limit; i++ {
		go worker()
	}

	for index := 0; index < len(items); index++ {
		indexCh <- index
	}
	close(indexCh)
	wg.Wait()

	if firstErr != nil {
		return results, firstErr
	}
	return results, nil
}

package async

import "sync"

// SettledResult holds the outcome of a single task processed by PSettled. When
// Status is "fulfilled" the task succeeded and Value carries its result with a
// nil Reason. When Status is "rejected" the task failed, Reason carries the
// error and Value is the zero value of T. This mirrors the TypeScript
// SettledResult discriminated union returned by Promise.allSettled style helpers.
type SettledResult[T any] struct {
	Status string
	Value  T
	Reason error
}

// PSettled runs every task and returns each task's settled result in input
// order, never returning an error of its own: failures are captured as rejected
// SettledResult entries. An optional concurrency limit caps how many tasks run
// at once; when limit is zero or negative it is treated as unlimited (the number
// of tasks), matching the TypeScript pSettled utility.
//
// An empty tasks slice returns an empty result slice immediately.
//
// Example:
//
//	results := PSettled([]func() (int, error){
//	    func() (int, error) { return 1, nil },
//	    func() (int, error) { return 0, errors.New("nope") },
//	})
//	// results[0].Status == "fulfilled", results[1].Status == "rejected"
func PSettled[T any](tasks []func() (T, error), limit ...int) []SettledResult[T] {
	results := make([]SettledResult[T], len(tasks))
	if len(tasks) == 0 {
		return results
	}

	effectiveLimit := len(tasks)
	if len(limit) > 0 && limit[0] > 0 {
		effectiveLimit = limit[0]
	}
	if effectiveLimit > len(tasks) {
		effectiveLimit = len(tasks)
	}

	var wg sync.WaitGroup
	indexCh := make(chan int)

	worker := func() {
		defer wg.Done()
		for index := range indexCh {
			value, err := tasks[index]()
			if err != nil {
				results[index] = SettledResult[T]{Status: "rejected", Reason: err}
			} else {
				results[index] = SettledResult[T]{Status: "fulfilled", Value: value}
			}
		}
	}

	wg.Add(effectiveLimit)
	for i := 0; i < effectiveLimit; i++ {
		go worker()
	}

	for index := 0; index < len(tasks); index++ {
		indexCh <- index
	}
	close(indexCh)
	wg.Wait()

	return results
}

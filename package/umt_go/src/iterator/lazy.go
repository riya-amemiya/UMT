// Package iterator provides lazy iterator helpers that mirror the TypeScript
// Iterator module. Sequences are modeled with the standard library iter.Seq
// type, so values are produced on demand and consumers may stop early.
package iterator

import "iter"

// LazyFilter lazily filters values from a sequence, yielding only the values
// for which predicate returns true. The predicate receives the value together
// with its zero-based index in the source sequence; the index advances for
// every source value, regardless of whether it is kept. Evaluation is lazy:
// the predicate runs only as values are pulled from the returned sequence, and
// stopping the consumer stops the source.
//
// Example:
//
//	evens := LazyFilter(slices.Values([]int{1, 2, 3, 4}), func(n, _ int) bool {
//	    return n%2 == 0
//	})
//	slices.Collect(evens) // [2 4]
func LazyFilter[T any](seq iter.Seq[T], predicate func(value T, index int) bool) iter.Seq[T] {
	return func(yield func(T) bool) {
		index := 0
		for value := range seq {
			if predicate(value, index) {
				if !yield(value) {
					return
				}
			}
			index++
		}
	}
}

// LazyMap lazily maps values from a sequence, yielding fn applied to each
// value. The mapping function receives the value together with its zero-based
// index in the source sequence. Evaluation is lazy: fn runs only as values are
// pulled from the returned sequence, and stopping the consumer stops the
// source.
//
// Example:
//
//	doubled := LazyMap(slices.Values([]int{1, 2, 3}), func(n, _ int) int {
//	    return n * 2
//	})
//	slices.Collect(doubled) // [2 4 6]
func LazyMap[T, U any](seq iter.Seq[T], fn func(value T, index int) U) iter.Seq[U] {
	return func(yield func(U) bool) {
		index := 0
		for value := range seq {
			if !yield(fn(value, index)) {
				return
			}
			index++
		}
	}
}

// LazyTake lazily yields at most the first n values from a sequence. Once n
// values have been produced it stops pulling from the source, so infinite or
// expensive sequences are only consumed as far as needed. A value of n that is
// zero or negative yields nothing.
//
// Example:
//
//	first3 := LazyTake(slices.Values([]int{1, 2, 3, 4, 5}), 3)
//	slices.Collect(first3) // [1 2 3]
func LazyTake[T any](seq iter.Seq[T], n int) iter.Seq[T] {
	return func(yield func(T) bool) {
		count := 0
		for value := range seq {
			if count >= n {
				return
			}
			if !yield(value) {
				return
			}
			count++
		}
	}
}

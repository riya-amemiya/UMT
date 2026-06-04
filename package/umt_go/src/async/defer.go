package async

import "sync"

// Deferred is a one-shot future whose resolution (value) or rejection (error)
// can be triggered externally, mirroring the TypeScript defer utility that
// exposes a promise together with its resolve and reject callbacks.
//
// The first call to Resolve or Reject wins; subsequent calls are ignored, just
// like a JavaScript promise which settles exactly once. Await blocks until the
// deferred has settled and then returns the stored value and error.
//
// Example:
//
//	d := NewDeferred[int]()
//	d.Resolve(42)
//	value, err := d.Await() // value == 42, err == nil
type Deferred[T any] struct {
	done  chan struct{}
	once  sync.Once
	value T
	err   error
}

// NewDeferred creates a new unsettled Deferred.
//
// Example:
//
//	d := NewDeferred[string]()
//	go func() { d.Resolve("delayed") }()
//	value, _ := d.Await() // value == "delayed"
func NewDeferred[T any]() *Deferred[T] {
	return &Deferred[T]{done: make(chan struct{})}
}

// Resolve settles the deferred with a value. Only the first settlement (Resolve
// or Reject) takes effect.
func (d *Deferred[T]) Resolve(value T) {
	d.once.Do(func() {
		d.value = value
		close(d.done)
	})
}

// Reject settles the deferred with an error. Only the first settlement (Resolve
// or Reject) takes effect.
func (d *Deferred[T]) Reject(reason error) {
	d.once.Do(func() {
		d.err = reason
		close(d.done)
	})
}

// Await blocks until the deferred settles, then returns its value and error.
// When the deferred was resolved the error is nil; when it was rejected the
// value is the zero value of T.
func (d *Deferred[T]) Await() (T, error) {
	<-d.done
	return d.value, d.err
}

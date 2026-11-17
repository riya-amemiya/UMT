import { safeExecute, type Result } from "@/Error/safeExecute";

/**
A class to handle pipeline processing
Allows chaining transformations in a fluent interface
@template T Type of the current value
*/
export class Pipe<T> {
  /**
  @param value Current value in the pipe
  */
  constructor(private readonly value: T) {}

  /**
  Applies a transformation function and returns a new Pipe instance
  @param fn Transformation function to apply
  @returns New Pipe instance with transformed value
  */
  map<U>(function_: (input: T) => U): Pipe<U> {
    return new Pipe(function_(this.value));
  }

  /**
  Applies a transformation function only if the condition is met
  @param predicate Condition function
  @param fn Transformation function to apply if condition is met
  @returns New Pipe instance with conditionally transformed value
  */
  when<U>(
    predicate: (input: T) => boolean,
    function_: (input: T) => U,
  ): Pipe<U | T> {
    return predicate(this.value)
      ? new Pipe(function_(this.value))
      : new Pipe(this.value);
  }

  /**
  Executes a side effect without changing the value
  @param fn Function to execute as a side effect
  @returns Same Pipe instance
  */
  tap(function_: (input: T) => void): Pipe<T> {
    function_(this.value);
    return this;
  }

  /**
  Strictly filters the value based on a predicate function
  Throws an error if the predicate returns false
  @param predicate Condition function that determines if value should be filtered
  @returns New Pipe instance with filtered value and narrowed type
  @throws Error if the predicate returns false
  */
  filterStrict<U extends T>(predicate: (input: T) => input is U): Pipe<U> {
    if (predicate(this.value)) {
      return new Pipe(this.value);
    }
    throw new Error("Value did not match filter predicate");
  }

  /**
  Filters the value based on a predicate function
  Returns a default value if the predicate returns false
  @param predicate Condition function that determines if value should be filtered
  @param defaultValue Default value to use if predicate returns false
  @returns New Pipe instance with filtered value or default value
  */
  filterWithDefault<U extends T>(
    predicate: (input: T) => input is U,
    defaultValue: U,
  ): Pipe<U> {
    return new Pipe(predicate(this.value) ? this.value : defaultValue);
  }

  /**
  Filters the value based on a predicate function
  Returns a Result type containing either the filtered value or an error
  @param predicate Condition function that determines if value should be filtered
  @returns New Pipe instance with Result containing filtered value or error
  */
  filterResult<U extends T, E extends Error = Error>(
    predicate: (input: T) => input is U,
  ): Pipe<Result<U, E>> {
    return new Pipe(
      safeExecute<U, E>(() => {
        if (predicate(this.value)) {
          return this.value;
        }
        throw new Error("Value did not match filter predicate");
      }),
    );
  }

  /**
  Terminates the pipeline and returns the final value
  @returns Final result of the pipeline processing
  */
  end(): T {
    return this.value;
  }
}

/**
Creates a new Pipe instance with an initial value
@param initialValue Initial value for the pipeline
@returns New Pipe instance
*/
export function pipe<T>(initialValue: T): Pipe<T>;
export function pipe<A, B>(initialValue: A, function1: (input: A) => B): B;
export function pipe<A, B, C>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
): C;
export function pipe<A, B, C, D>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
): D;
export function pipe<A, B, C, D, E>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
): E;
export function pipe<A, B, C, D, E, F>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
): F;
export function pipe<A, B, C, D, E, F, G>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
): G;
export function pipe<A, B, C, D, E, F, G, H>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
): H;
export function pipe<A, B, C, D, E, F, G, H, I>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
): I;
export function pipe<A, B, C, D, E, F, G, H, I, J>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
): J;
export function pipe<A, B, C, D, E, F, G, H, I, J, K>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
): K;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
): L;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
): M;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
): N;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
): O;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
): P;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
  function16: (input: P) => Q,
): Q;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
  function16: (input: P) => Q,
  function17: (input: Q) => R,
): R;
export function pipe<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
  function16: (input: P) => Q,
  function17: (input: Q) => R,
  function18: (input: R) => S,
): S;
export function pipe<
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
  function16: (input: P) => Q,
  function17: (input: Q) => R,
  function18: (input: R) => S,
  function19: (input: S) => T,
): T;
export function pipe<
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
>(
  initialValue: A,
  function1: (input: A) => B,
  function2: (input: B) => C,
  function3: (input: C) => D,
  function4: (input: D) => E,
  function5: (input: E) => F,
  function6: (input: F) => G,
  function7: (input: G) => H,
  function8: (input: H) => I,
  function9: (input: I) => J,
  function10: (input: J) => K,
  function11: (input: K) => L,
  function12: (input: L) => M,
  function13: (input: M) => N,
  function14: (input: N) => O,
  function15: (input: O) => P,
  function16: (input: P) => Q,
  function17: (input: Q) => R,
  function18: (input: R) => S,
  function19: (input: S) => T,
  function20: (input: T) => U,
): U;
export function pipe(
  initialValue: unknown,
  ...functions: ((input: unknown) => unknown)[]
): unknown {
  if (functions.length === 0) {
    return new Pipe(initialValue);
  }
  let value = initialValue;
  for (const function_ of functions) {
    value = function_(value);
  }
  return value;
}

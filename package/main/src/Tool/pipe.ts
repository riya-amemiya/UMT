import { safeExecute, type Result } from "@/Error/safeExecute";

/**
 * A class to handle pipeline processing
 * Allows chaining transformations in a fluent interface
 * @template T Type of the current value
 */
export class Pipe<T> {
  /**
   * @param value Current value in the pipe
   */
  constructor(private readonly value: T) {}

  /**
   * Applies a transformation function and returns a new Pipe instance
   * @param fn Transformation function to apply
   * @returns New Pipe instance with transformed value
   */
  map<U>(function_: (input: T) => U): Pipe<U> {
    return new Pipe(function_(this.value));
  }

  /**
   * Applies a transformation function only if the condition is met
   * @param predicate Condition function
   * @param fn Transformation function to apply if condition is met
   * @returns New Pipe instance with conditionally transformed value
   */
  when<U>(
    predicate: (input: T) => boolean,
    function_: (input: T) => U,
  ): Pipe<U | T> {
    return predicate(this.value)
      ? new Pipe(function_(this.value))
      : new Pipe(this.value as unknown as U | T);
  }

  /**
   * Executes a side effect without changing the value
   * @param fn Function to execute as a side effect
   * @returns Same Pipe instance
   */
  tap(function_: (input: T) => void): Pipe<T> {
    function_(this.value);
    return this;
  }

  /**
   * Strictly filters the value based on a predicate function
   * Throws an error if the predicate returns false
   * @param predicate Condition function that determines if value should be filtered
   * @returns New Pipe instance with filtered value and narrowed type
   * @throws Error if the predicate returns false
   */
  filterStrict<U extends T>(predicate: (input: T) => input is U): Pipe<U> {
    if (predicate(this.value)) {
      return new Pipe(this.value);
    }
    throw new Error("Value did not match filter predicate");
  }

  /**
   * Filters the value based on a predicate function
   * Returns a default value if the predicate returns false
   * @param predicate Condition function that determines if value should be filtered
   * @param defaultValue Default value to use if predicate returns false
   * @returns New Pipe instance with filtered value or default value
   */
  filterWithDefault<U extends T>(
    predicate: (input: T) => input is U,
    defaultValue: U,
  ): Pipe<U> {
    return predicate(this.value)
      ? new Pipe(this.value)
      : new Pipe(defaultValue);
  }

  /**
   * Filters the value based on a predicate function
   * Returns a Result type containing either the filtered value or an error
   * @param predicate Condition function that determines if value should be filtered
   * @returns New Pipe instance with Result containing filtered value or error
   */
  filterResult<U extends T>(
    predicate: (input: T) => input is U,
  ): Pipe<Result<U, Error>> {
    return new Pipe(
      safeExecute<U, Error>(() => {
        if (predicate(this.value)) {
          return this.value;
        }
        throw new Error("Value did not match filter predicate");
      }),
    );
  }

  /**
   * Terminates the pipeline and returns the final value
   * @returns Final result of the pipeline processing
   */
  end(): T {
    return this.value;
  }
}

/**
 * Creates a new Pipe instance with an initial value
 * @param initialValue Initial value for the pipeline
 * @returns New Pipe instance
 */
export function pipe<T>(initialValue: T): Pipe<T> {
  return new Pipe(initialValue);
}

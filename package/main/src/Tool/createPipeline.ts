/**
 * Interface representing a pipeline.
 * When called without arguments, returns the stored value.
 * When called with a function argument, applies the function and generates a new Pipeline instance.
 * @template T The type of value being processed in the pipeline
 */
export interface Pipeline<T> {
  (): T;
  <U>(transformer: (input: T) => U): Pipeline<U>;
}

/**
 * Function that creates a Pipeline instance
 * @template T The type of value being processed in the pipeline
 * @param initialValue Initial value to start the pipeline
 * @returns Pipeline instance
 * @example const pipeline = createPipeline(1);
 * pipeline(); // 1
 * pipeline((x) => x + 1)(); // 2
 */
export const createPipeline: <T>(initialValue: T) => Pipeline<T> = <T>(
  initialValue: T,
) =>
  // Accepts a transformer function as an argument and returns a new Pipeline.
  // If no argument is provided, returns the stored value (initialValue).
  (<U>(transformer?: (input: T) => U) =>
    transformer
      ? createPipeline(transformer(initialValue))
      : initialValue) as Pipeline<T>;

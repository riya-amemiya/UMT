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
export function createPipeline<T>(initialValue: T): Pipeline<T> {
  function pipeline(): T;
  function pipeline<U>(transformer: (input: T) => U): Pipeline<U>;
  function pipeline<U>(transformer?: (input: T) => U): T | Pipeline<U> {
    return transformer
      ? createPipeline(transformer(initialValue))
      : initialValue;
  }
  return pipeline;
}

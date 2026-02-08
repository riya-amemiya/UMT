/**
 * A deferred promise with externally accessible resolve and reject
 */
export interface Deferred<T> {
  promise: Promise<T>;
  resolve: (value: T | PromiseLike<T>) => void;
  reject: (reason?: unknown) => void;
}

/**
 * Creates a deferred promise whose resolve and reject can be
 * called externally
 * @returns {Deferred<T>} An object with promise, resolve, and reject
 * @example
 * const d = defer<number>();
 * d.resolve(42);
 * const value = await d.promise;
 */
export const defer = <T>(): Deferred<T> => {
  let resolve!: (value: T | PromiseLike<T>) => void;
  let reject!: (reason?: unknown) => void;

  const promise = new Promise<T>((resolver, rejector) => {
    resolve = resolver;
    reject = rejector;
  });

  return { promise, resolve, reject };
};

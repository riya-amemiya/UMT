/**
Creates a pipe operator that applies a function to each element of an array,
or to the value itself when the input is not an array
@param {(value: T extends readonly (infer V)[] ? V : T) => U} function_ - The function to apply
@returns {(input: T) => T extends readonly unknown[] ? U[] : U} An operator that transforms each element or the value
@example
pipe([1, 2, 3], transform((x) => x * 2)); // [2, 4, 6]
pipe(2, transform((x) => x * 2)); // 4
*/
export function transform<T, U>(
  function_: (value: NoInfer<T extends readonly (infer V)[] ? V : T>) => U,
): (input: T) => T extends readonly unknown[] ? U[] : U;
export function transform(
  function_: (value: unknown) => unknown,
): (input: unknown) => unknown {
  return (input: unknown): unknown =>
    Array.isArray(input)
      ? input.map((value) => function_(value))
      : function_(input);
}

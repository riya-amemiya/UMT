/**
 * Function validation core module
 * Provides a validator for function values along with an `implement()`
 * helper that wraps a concrete function with runtime validation of its
 * inputs and output.
 *
 * The runtime validator only enforces that the value is a function, since
 * argument and return-value contracts can only be checked when the function
 * is actually invoked. Use `.implement()` (or call the validator's
 * `implement` property directly) to create a wrapped function that asserts
 * the schema on every call.
 */

import type { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

export type FunctionAnyValidator = (
  // biome-ignore lint/suspicious/noExplicitAny: validator signatures vary
  value: any,
) => ValidateCoreReturnType<unknown>;

export type FunctionExtractValidatedType<V> = V extends (value: never) => {
  type: infer T;
}
  ? ValidateType<T>
  : never;

export type InferInputs<Inputs> = Inputs extends readonly FunctionAnyValidator[]
  ? { [K in keyof Inputs]: FunctionExtractValidatedType<Inputs[K]> }
  : // biome-ignore lint/suspicious/noExplicitAny: open signature when no schema provided
    any[];

export type InferOutput<Output> = Output extends FunctionAnyValidator
  ? FunctionExtractValidatedType<Output>
  : // biome-ignore lint/suspicious/noExplicitAny: open signature when no schema provided
    any;

/**
 * Function schema definition. `input` describes positional parameters and
 * `output` describes the return type. Both are optional; when omitted the
 * validator only enforces that the value is callable.
 */
export interface FunctionSchema {
  input?: readonly FunctionAnyValidator[];
  output?: FunctionAnyValidator;
}

export type ExtractInput<S> = S extends { input: infer I } ? I : undefined;
export type ExtractOutput<S> = S extends { output: infer O } ? O : undefined;

/**
 * Inferred function signature from a `FunctionSchema`. Used by callers via
 * `SchemaToInterface` and as the return type of `implement()`.
 * @template Inputs - Tuple of validators describing positional parameters
 * @template Output - Validator describing the return value
 */
export type InferFunction<Inputs, Output> = (
  ...arguments_: InferInputs<Inputs>
) => InferOutput<Output>;

/**
 * Validator return type. Carries the inferred function signature through
 * the `type` field so `SchemaToInterface` can recover it.
 * @template Inputs - Tuple of validators describing positional parameters
 * @template Output - Validator describing the return value
 */
export interface FunctionReturnType<Inputs, Output> {
  validate: boolean;
  message: string;
  type: InferFunction<Inputs, Output>;
}

/**
 * Validator value enriched with an `implement()` method that returns a
 * runtime-checked wrapper of a concrete function matching the schema.
 * @template Inputs - Tuple of validators describing positional parameters
 * @template Output - Validator describing the return value
 */
export interface FunctionValidator<Inputs, Output> {
  (value: InferFunction<Inputs, Output>): FunctionReturnType<Inputs, Output>;
  implement: (
    function__: InferFunction<Inputs, Output>,
  ) => InferFunction<Inputs, Output>;
}

/**
 * Creates a function validator. When invoked the validator only checks that
 * the value is callable; use `.implement()` on the returned validator to
 * obtain a runtime-checked wrapper that asserts the schema for each call.
 * @template S - Function schema type, captured for input/output inference
 * @param {S} [schema] - Function schema definition
 * @param {string} [message] - Custom error message for type validation
 * @returns {FunctionValidator} - Validator augmented with `implement()`
 */
export const function_ = <const S extends FunctionSchema = FunctionSchema>(
  schema?: S,
  message?: string,
): FunctionValidator<ExtractInput<S>, ExtractOutput<S>> => {
  type Inputs = ExtractInput<S>;
  type Output = ExtractOutput<S>;
  const inputs = schema?.input;
  const output = schema?.output;

  const validator = ((value: InferFunction<Inputs, Output>) => {
    if (typeof value !== "function") {
      return {
        validate: false,
        message: message ?? "",
        type: value,
      };
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  }) as FunctionValidator<Inputs, Output>;

  validator.implement = (function__) => {
    if (typeof function__ !== "function") {
      throw new TypeError(message ?? "value is not a function");
    }
    const wrapped = (...arguments_: InferInputs<Inputs>) => {
      if (inputs) {
        for (const [index, inputValidator] of inputs.entries()) {
          const result = inputValidator(arguments_[index]);
          if (!result.validate) {
            throw new TypeError(
              result.message ||
                `function input at index ${index} failed validation`,
            );
          }
        }
      }
      const returnValue = (
        function__ as unknown as (...callArguments: unknown[]) => unknown
      )(...(arguments_ as unknown as unknown[]));
      if (output) {
        const result = output(returnValue);
        if (!result.validate) {
          throw new TypeError(
            result.message || "function output failed validation",
          );
        }
      }
      return returnValue as InferOutput<Output>;
    };
    return wrapped;
  };

  return validator;
};

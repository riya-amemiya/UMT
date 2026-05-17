/**
 * Template literal validation core module
 * Provides a validator that checks whether a string matches a template
 * literal pattern composed of string fragments and primitive validators
 * (string / number / boolean / bigint). The runtime check is performed by
 * an auto-generated regular expression assembled from the parts, while the
 * inferred type is the corresponding TypeScript template literal type.
 */

import {
  attachStandard,
  type StandardSchemaV1,
} from "@/Validate/standardSchema";
import type { ValidateType } from "@/Validate/type";

// biome-ignore lint/suspicious/noExplicitAny: validator signatures vary
export type TemplateLiteralAnyValidator = (value?: any) => { type: unknown };

/**
 * Allowed parts of a template literal definition. Each element is either a
 * string literal that must appear verbatim, or a primitive validator whose
 * accepted shape is converted to a regex fragment at construction time.
 */
export type TemplateLiteralPart = string | TemplateLiteralAnyValidator;

export type ExtractValidatorTag<V> = V extends (value: never) => {
  type: infer T;
}
  ? T
  : never;

export type TagToTemplate<T> = T extends "string"
  ? string
  : T extends "number"
    ? number
    : T extends "boolean"
      ? boolean
      : T extends "bigint"
        ? bigint
        : ValidateType<T>;

export type PartToTemplate<P> = P extends string
  ? P
  : TagToTemplate<ExtractValidatorTag<P>>;

/**
 * Builds the template literal type produced by joining `Parts`. Each part is
 * mapped to either its literal string value or to the runtime type that the
 * corresponding validator accepts.
 */
export type BuildTemplateLiteral<Parts extends readonly TemplateLiteralPart[]> =
  Parts extends readonly [
    infer Head,
    ...infer Tail extends readonly TemplateLiteralPart[],
  ]
    ? `${Extract<PartToTemplate<Head>, string | number | bigint | boolean>}${BuildTemplateLiteral<Tail>}`
    : "";

/**
 * Return type produced by a `templateLiteral` validator. Preserves the
 * literal template string type through the `type` field so consumers like
 * `union()`, `intersection()`, and `SchemaToInterface` can recover it.
 * @template T - The inferred template literal type
 */
export interface TemplateLiteralReturnType<T extends string> {
  validate: boolean;
  message: string;
  type: T;
}

const escapeRegex = (input: string): string =>
  input.replaceAll(/[$()*+.?[\\\]^{|}]/g, String.raw`\$&`);

const tagToPattern = (tag: unknown): string => {
  switch (tag) {
    case "string": {
      return ".*?";
    }
    case "number": {
      return String.raw`-?(?:\d+\.\d+|\d+(?:\.\d*)?|\.\d+)`;
    }
    case "boolean": {
      return "(?:true|false)";
    }
    case "bigint": {
      return String.raw`-?\d+`;
    }
    default: {
      return ".+?";
    }
  }
};

const detectValidatorTag = (
  validator: TemplateLiteralAnyValidator,
): unknown => {
  const result = validator();
  return result?.type;
};

/**
 * Creates a validator that checks whether a value matches a template literal
 * pattern. Each part is either a string literal that must appear verbatim or
 * a primitive validator (string / number / boolean / bigint) that contributes
 * a regex fragment.
 * @template Parts - Tuple describing the template parts
 * @param {Parts} parts - Tuple of literal strings and primitive validators
 * @param {string} [message] - Custom error message for validation failure
 * @returns {Function} - Validator function for template literal strings
 */
export const templateLiteral = <
  const Parts extends readonly TemplateLiteralPart[],
>(
  parts: Parts,
  message?: string,
): ((
  value: BuildTemplateLiteral<Parts>,
) => TemplateLiteralReturnType<BuildTemplateLiteral<Parts>>) &
  StandardSchemaV1<
    BuildTemplateLiteral<Parts>,
    BuildTemplateLiteral<Parts>
  > => {
  let pattern = "^";
  for (const part of parts) {
    pattern +=
      typeof part === "string"
        ? escapeRegex(part)
        : `(?:${tagToPattern(detectValidatorTag(part))})`;
  }
  pattern += "$";
  const regex = new RegExp(pattern);

  const templateValidator = (
    value: BuildTemplateLiteral<Parts>,
  ): TemplateLiteralReturnType<BuildTemplateLiteral<Parts>> => {
    if (typeof value !== "string" || !regex.test(value)) {
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
  };
  return attachStandard<
    BuildTemplateLiteral<Parts>,
    BuildTemplateLiteral<Parts>,
    typeof templateValidator
  >(templateValidator);
};

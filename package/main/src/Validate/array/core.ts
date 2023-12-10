import { ValidateCoreReturnType } from "@/Validate/type";
import { isArray } from "@/Validate/isArray";

export const array = <A>(
  option: Partial<
    Record<
      | "string"
      | "number"
      | "bigint"
      | "boolean"
      | "symbol"
      | "undefined"
      | "object"
      | "function",
      // biome-ignore lint/suspicious/noExplicitAny: <explanation>
      (value: any) => ValidateCoreReturnType<any>
    >
  >,
  message?: string,
) => {
  return (values: A[]) => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message || "",
        type: values,
      };
    }

    for (const value of values) {
      const validater = option[typeof value];
      if (!validater?.(value).validate) {
        return {
          validate: false,
          message: message || "",
          type: values,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: values,
    };
  };
};

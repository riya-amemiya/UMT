import { isObject } from "@/Validate/isObject";
import { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

export const object = <
  T extends {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    [key: string]: (value: any) => ValidateCoreReturnType<any>;
  },
>(
  option: T,
  message?: string,
) => {
  return (
    value: {
      [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
    },
  ) => {
    if (!isObject(value)) {
      return {
        validate: false,
        message: message || "",
        type: value,
      };
    }
    for (const validate in option) {
      if (!option[validate](value[validate] as object).validate) {
        return {
          validate: false,
          message: option[validate](value as object).message,
          type: value,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: value,
    };
  };
};

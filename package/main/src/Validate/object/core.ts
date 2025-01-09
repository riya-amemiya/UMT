import { isDictionaryObject } from "@/Validate/isDictionaryObject";
import type { ValidateCoreReturnType, ValidateType } from "@/Validate/type";

export const object = <
  T extends {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    [key: string]: (value: any) => ValidateCoreReturnType<any>;
  },
>(
  option: T = {} as T,
  message?: string,
) => {
  return (
    value: {
      [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]>;
    },
  ): {
    validate: boolean;
    message: string;
    type: { [key in keyof T]: ValidateType<ReturnType<T[key]>["type"]> };
  } => {
    if (!isDictionaryObject(value)) {
      return {
        validate: false,
        message: message || "",
        type: value,
      };
    }
    for (const validate in option) {
      if (!option[validate](value[validate]).validate) {
        return {
          validate: false,
          message: option[validate](value).message,
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

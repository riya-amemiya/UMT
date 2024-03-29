import { isArray } from "@/Validate/isArray";
import { isNotEmpty } from "@/Validate/isNotEmpty";
import type {
  Types,
  ValidateCoreReturnType,
  ValidateType,
} from "@/Validate/type";

export const array = <
  A extends string | number | boolean,
  O extends {
    [P in Types<A>]: (
      value: ValidateType<P>,
    ) => ValidateCoreReturnType<ValidateType<P>>;
  } = {
    [P in Types<A>]: (
      value: ValidateType<P>,
    ) => ValidateCoreReturnType<ValidateType<P>>;
  },
>(
  option: O = {} as O,
  message?: string,
) => {
  return (values: A[]): ValidateCoreReturnType<A[]> => {
    if (!isArray(values)) {
      return {
        validate: false,
        message: message || "",
        type: values,
      };
    }
    if (isNotEmpty(option)) {
      for (const value of values) {
        const validater = option[typeof value as Types<A>];
        if (!validater?.(value as never).validate) {
          return {
            validate: false,
            message: validater?.(value as never).message || "",
            type: values,
          };
        }
      }
    }
    return {
      validate: true,
      message: "",
      type: values,
    };
  };
};

import type {
  Types,
  ValidateCoreReturnType,
  ValidateReturnType,
} from "@/Validate/type";

export const core =
  <T>(type: Types<T>) =>
  <O extends ValidateReturnType<T>[]>(
    value: T,
    option: O = [] as unknown as O,
    message?: string,
  ): ValidateCoreReturnType<T> => {
    // biome-ignore lint/suspicious/useValidTypeof: <explanation>
    if (typeof value !== type) {
      return {
        validate: false,
        message: message || "",
        type,
      };
    }
    for (const validate of option) {
      if (!validate.validate(value)) {
        return {
          validate: false,
          message: validate.message || "",
          type,
        };
      }
    }
    return {
      validate: true,
      message: "",
      type,
    };
  };

import type { ValidateCoreReturnType } from "@/Validate/type";

export const optional = <T>(
  validator: (value: T) => ValidateCoreReturnType<T>,
): ((value?: T) => ValidateCoreReturnType<T | undefined>) => {
  const optionalValidator = (
    value?: T,
  ): ValidateCoreReturnType<T | undefined> => {
    if (value === undefined) {
      return {
        validate: true,
        message: "",
        type: "undefined",
      };
    }
    const result = validator(value);
    return result;
  };

  return optionalValidator;
};

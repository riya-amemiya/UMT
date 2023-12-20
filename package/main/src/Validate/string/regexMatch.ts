import { ValidateReturnType } from "@/Validate/type";

export const regexMatch = (
  pattern: RegExp,
  message?: string,
): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value: string) => pattern.test(value),
  };
};

import { ValidateReturnType } from "@/Validate/type";

export const maxLength = (
  maxLength: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length <= maxLength,
});

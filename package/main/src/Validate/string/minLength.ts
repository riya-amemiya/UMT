import type { ValidateReturnType } from "@/Validate/type";

export const minLength = (
  minLength: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length >= minLength,
});

import { ValidateReturnType } from "@/Validate/type";

export const length_ = (
  length: number,
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => value.length === length,
});

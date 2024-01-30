import type { ValidateReturnType } from "@/Validate/type";

export const maxValue = (
  maxValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value <= maxValue,
  };
};

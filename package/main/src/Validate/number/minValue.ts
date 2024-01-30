import type { ValidateReturnType } from "@/Validate/type";

export const minValue = (
  minValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value >= minValue,
  };
};

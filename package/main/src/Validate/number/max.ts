import { ValidateReturnType } from "@/Validate/type";

export const max = (
  maxValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value <= maxValue,
  };
};

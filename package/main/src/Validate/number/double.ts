import { isDouble } from "@/Validate/isDouble";
import type { ValidateReturnType } from "@/Validate/type";

export const double = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isDouble(value),
  };
};

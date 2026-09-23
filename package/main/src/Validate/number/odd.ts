import { isDouble } from "@/Validate/isDouble";
import type { ValidateReturnType } from "@/Validate/type";

export const odd = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => {
      return !isDouble(value, false) && value % 2 !== 0;
    },
  };
};

import { isDouble } from "@/Validate/isDouble";
import type { ValidateReturnType } from "@/Validate/type";

export const odd = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => {
      if (isDouble(value, false)) {
        return false;
      }
      return value % 2 !== 0;
    },
  };
};

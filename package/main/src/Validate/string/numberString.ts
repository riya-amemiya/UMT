import { isNumber } from "@/Validate/isNumber";
import type { ValidateReturnType } from "@/Validate/type";

export const numberString = (message?: string): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value) => isNumber(value),
  };
};

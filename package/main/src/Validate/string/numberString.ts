import { ValidateReturnType } from "@/Validate/type";
import { isNumber } from "@/Validate/isNumber";

export const numberString = (message?: string): ValidateReturnType<string> => {
  return {
    type: "string",
    message,
    validate: (value) => isNumber(value),
  };
};

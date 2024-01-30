import { isPrimeNumber } from "@/Validate/isPrimeNumber";
import type { ValidateReturnType } from "@/Validate/type";

export const prime = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isPrimeNumber(value),
  };
};

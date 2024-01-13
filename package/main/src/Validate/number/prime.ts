import { isPrimeNumber } from "@/Validate/isPrimeNumber";
import { ValidateReturnType } from "@/Validate/type";

export const prime = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isPrimeNumber(value),
  };
};

import { ValidateReturnType } from "@/Validate/type";
import { isPrimeNumber } from "../isPrimeNumber";

export const prime = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => isPrimeNumber(value),
  };
};

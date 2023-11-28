import { ValidateReturnType } from "@/Validate/type";

export const min = (
  minValue: number,
  message?: string,
): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value >= minValue,
  };
};

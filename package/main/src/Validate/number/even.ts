import type { ValidateReturnType } from "@/Validate/type";

export const even = (message?: string): ValidateReturnType<number> => {
  return {
    type: "number",
    message,
    validate: (value) => value % 2 === 0,
  };
};

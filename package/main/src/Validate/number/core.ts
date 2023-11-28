import { core } from "@/Validate/core";
import { ValidateReturnType } from "@/Validate/type";

export const number = <T extends ValidateReturnType<number>[]>(
  option: T,
  message?: string,
) => {
  return (value: number) => core<number>("number")(value, option, message);
};

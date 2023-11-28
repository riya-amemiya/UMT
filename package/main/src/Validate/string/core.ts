import { core } from "@/Validate/core";
import { ValidateReturnType } from "@/Validate/type";

export const string =
  <T extends ValidateReturnType<string>[]>(option: T, message?: string) =>
  (value: string) =>
    core<string>("string")(value, option, message);

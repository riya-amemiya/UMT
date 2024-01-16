import { core } from "@/Validate/core";
import type { ValidateCoreReturnType } from "@/Validate/type";

export const boolean = (message?: string) => {
  return (value: boolean): ValidateCoreReturnType<boolean> =>
    core<boolean>("boolean")(value, [], message);
};

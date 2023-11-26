import { ValidateReturnType } from "@/Validate/type";

export const string = <T extends ValidateReturnType<string>[]>(
  option: T,
  message?: string,
) => {
  return (
    value: string,
  ): {
    validate: boolean;
    message: string;
    type: "string";
  } => {
    if (typeof value !== "string") {
      return {
        validate: false,
        message: message || "",
        type: "string",
      };
    }
    for (const validate of option) {
      if (!validate.validate(value)) {
        return {
          validate: false,
          message: validate.message || "",
          type: "string",
        };
      }
    }
    return {
      validate: true,
      message: "",
      type: "string",
    };
  };
};

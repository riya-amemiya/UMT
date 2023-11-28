import { ValidateReturnType } from "@/Validate/type";

export const email = (message?: string): ValidateReturnType<string> => {
  // メールアドレスの正規表現
  const emailRegex =
    /^[\w+-]+(?:\.[\w+-]+)*@[\da-z]+(?:[.-][\da-z]+)*\.[a-z]{2,}$/iu;
  return {
    type: "string",
    message,
    validate: (value) => emailRegex.test(value),
  };
};

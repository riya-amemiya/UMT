import { ValidateReturnType } from "@/Validate/type";

export const uuid = (
  versions: (1 | 2 | 3 | 4 | 5)[] = [4],
  message?: string,
): ValidateReturnType<string> => ({
  type: "string",
  message,
  validate: (value) => {
    return versions.some((version) => {
      // バージョンごとの正規表現
      const versionRegex = new RegExp(
        `^[\\da-f]{8}-?[\\da-f]{4}-?${version}[\\da-f]{3}-?[89ab][\\da-f]{3}-?[\\da-f]{12}$`,
        "i",
      );
      return versionRegex.test(value);
    });
  },
});

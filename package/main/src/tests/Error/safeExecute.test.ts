import { safeExecute } from "@/Error/safeExecute";

describe("result function tests", () => {
  it("should return OkType for successful operations", () => {
    const successfulOperation = () => "test";
    const resultValue = safeExecute(successfulOperation);

    expect(resultValue).toHaveProperty("type", "success");
    expect(resultValue).toHaveProperty("value", "test");
  });

  it("should return ErrorType for operations that throw", () => {
    const errorOperation = () => {
      throw new Error("test error");
    };
    const resultValue = safeExecute(errorOperation);

    expect(resultValue).toHaveProperty("type", "err");
    expect(resultValue).toHaveProperty("error");
  });
});

import { result } from "@/Error/result";

describe("result function tests", () => {
  it("should return OkType for successful operations", () => {
    const successfulOperation = () => "test";
    const resultValue = result(successfulOperation);

    expect(resultValue).toHaveProperty("type", "ok");
    expect(resultValue).toHaveProperty("value", "test");
  });

  it("should return ErrorType for operations that throw", () => {
    const errorOperation = () => {
      throw new Error("test error");
    };
    const resultValue = result(errorOperation);

    expect(resultValue).toHaveProperty("type", "err");
    expect(resultValue).toHaveProperty("error");
  });
});

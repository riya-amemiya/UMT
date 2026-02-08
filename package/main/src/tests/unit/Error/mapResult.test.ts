import { mapResult } from "@/Error/mapResult";
import type { Result } from "@/Error/safeExecute";

describe("mapResult", () => {
  it("should transform the value of a success result", () => {
    const success: Result<number, Error> = {
      type: "success",
      value: 5,
    };
    const result = mapResult(success, (n) => n * 2);

    expect(result).toEqual({ type: "success", value: 10 });
  });

  it("should return the error unchanged for an error result", () => {
    const error: Result<number, string> = {
      type: "error",
      error: "something went wrong",
    };
    const result = mapResult(error, (n: number) => n * 2);

    expect(result).toEqual({
      type: "error",
      error: "something went wrong",
    });
  });

  it("should handle type transformation", () => {
    const success: Result<number, Error> = {
      type: "success",
      value: 42,
    };
    const result = mapResult(success, (n) => String(n));

    expect(result).toEqual({ type: "success", value: "42" });
  });

  it("should handle identity mapping", () => {
    const success: Result<string, Error> = {
      type: "success",
      value: "hello",
    };
    const result = mapResult(success, (v) => v);

    expect(result).toEqual({ type: "success", value: "hello" });
  });
});

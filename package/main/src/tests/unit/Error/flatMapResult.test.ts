import { flatMapResult } from "@/Error/flatMapResult";
import type { Result } from "@/Error/safeExecute";

describe("flatMapResult", () => {
  it("should chain success results", () => {
    const success: Result<number, string> = {
      type: "success",
      value: 5,
    };
    const result = flatMapResult(success, (n) => ({
      type: "success" as const,
      value: n * 2,
    }));

    expect(result).toEqual({ type: "success", value: 10 });
  });

  it("should return original error when input is an error", () => {
    const error: Result<number, string> = {
      type: "error",
      error: "original error",
    };
    const result = flatMapResult(error, (n: number) => ({
      type: "success" as const,
      value: n * 2,
    }));

    expect(result).toEqual({ type: "error", error: "original error" });
  });

  it("should return error from the mapping function", () => {
    const success: Result<number, string> = {
      type: "success",
      value: -1,
    };
    const result = flatMapResult(success, (n) =>
      n > 0
        ? { type: "success" as const, value: n }
        : { type: "error" as const, error: "negative" },
    );

    expect(result).toEqual({ type: "error", error: "negative" });
  });

  it("should support type transformation", () => {
    const success: Result<number, string> = {
      type: "success",
      value: 42,
    };
    const result = flatMapResult(success, (n) => ({
      type: "success" as const,
      value: String(n),
    }));

    expect(result).toEqual({ type: "success", value: "42" });
  });
});

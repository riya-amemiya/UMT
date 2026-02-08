import { matchResult } from "@/Error/matchResult";
import type { Result } from "@/Error/safeExecute";

describe("matchResult", () => {
  it("should call onSuccess for a success result", () => {
    const success: Result<number, Error> = {
      type: "success",
      value: 42,
    };
    const result = matchResult(success, {
      onSuccess: (v: number) => `Got ${v}`,
      onError: (e: Error) => `Failed: ${e.message}`,
    });

    expect(result).toBe("Got 42");
  });

  it("should call onError for an error result", () => {
    const error: Result<number, Error> = {
      type: "error",
      error: new Error("fail"),
    };
    const result = matchResult(error, {
      onSuccess: (v: number) => `Got ${v}`,
      onError: (e: Error) => `Failed: ${e.message}`,
    });

    expect(result).toBe("Failed: fail");
  });

  it("should return the correct type from onSuccess", () => {
    const success: Result<string, Error> = {
      type: "success",
      value: "hello",
    };
    const result = matchResult(success, {
      onSuccess: (v) => v.length,
      onError: () => -1,
    });

    expect(result).toBe(5);
  });

  it("should handle different return types", () => {
    const error: Result<number, string> = {
      type: "error",
      error: "not found",
    };
    const result = matchResult(error, {
      onSuccess: (v) => ({ found: true, value: v }),
      onError: (e) => ({ found: false, value: e }),
    });

    expect(result).toEqual({ found: false, value: "not found" });
  });
});

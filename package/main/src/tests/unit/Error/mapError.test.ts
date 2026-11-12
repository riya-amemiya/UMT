import { mapError } from "@/Error/mapError";
import type { Result } from "@/Error/safeExecute";

describe("mapError", () => {
  it("should transform the error of an error result", () => {
    const error: Result<number, string> = { type: "error", error: "fail" };
    const result = mapError(error, (message) => `wrapped: ${message}`);

    expect(result).toEqual({ type: "error", error: "wrapped: fail" });
  });

  it("should return the success unchanged for a success result", () => {
    const success: Result<number, string> = { type: "success", value: 5 };
    const mapper = jest.fn((message: string) => message.length);
    const result = mapError(success, mapper);

    expect(result).toBe(success);
    expect(mapper).not.toHaveBeenCalled();
  });

  it("should handle error type transformation", () => {
    const error: Result<number, string> = { type: "error", error: "boom" };
    const result = mapError(error, (message) => new Error(message));

    expect(result.type).toBe("error");
    expect(result.type === "error" && result.error.message).toBe("boom");
  });
});

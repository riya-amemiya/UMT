import type { Result } from "@/Error/safeExecute";
import { unwrapOr } from "@/Error/unwrapOr";

describe("unwrapOr", () => {
  it("should return the value of a success result", () => {
    const success: Result<number, Error> = { type: "success", value: 5 };
    expect(unwrapOr(success, 0)).toBe(5);
  });

  it("should return the default value for an error result", () => {
    const error: Result<number, string> = { type: "error", error: "fail" };
    expect(unwrapOr(error, 0)).toBe(0);
  });

  it("should return falsy success values instead of the default", () => {
    const success: Result<number | undefined, Error> = {
      type: "success",
      value: undefined,
    };
    expect(unwrapOr(success, 7)).toBeUndefined();
  });

  it("should allow a default of a different type", () => {
    const error: Result<number, Error> = {
      type: "error",
      error: new Error("fail"),
    };
    expect(unwrapOr(error, "none")).toBe("none");
  });
});

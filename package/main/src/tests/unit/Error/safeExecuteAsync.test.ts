import { safeExecuteAsync } from "@/Error/safeExecuteAsync";

describe("safeExecuteAsync", () => {
  describe("successful operations", () => {
    it("returns success for a resolved promise", async () => {
      const result = await safeExecuteAsync(async () => 42);
      expect(result).toEqual({ type: "success", value: 42 });
    });

    it("returns success for a sync return value", async () => {
      const result = await safeExecuteAsync(() => "ok");
      expect(result).toEqual({ type: "success", value: "ok" });
    });

    it("returns success with object value", async () => {
      const value = { key: "value" };
      const result = await safeExecuteAsync(async () => value);
      expect(result).toEqual({ type: "success", value });
    });
  });

  describe("error handling", () => {
    it("returns error for a rejected promise", async () => {
      const error = new Error("async fail");
      const result = await safeExecuteAsync(() => Promise.reject(error));
      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBe(error);
        expect(result.error.message).toBe("async fail");
      }
    });

    it("returns error for a sync throw", async () => {
      const error = new TypeError("sync fail");
      const result = await safeExecuteAsync(() => {
        throw error;
      });
      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBe(error);
      }
    });

    it("handles non-Error thrown values", async () => {
      const result = await safeExecuteAsync(() =>
        Promise.reject("string error"),
      );
      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBe("string error");
      }
    });
  });
});

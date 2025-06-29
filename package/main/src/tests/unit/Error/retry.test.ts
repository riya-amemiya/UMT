import { retry } from "@/Error/retry";

describe("retry function", () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  describe("successful operations", () => {
    it("should return the result on first successful attempt", async () => {
      const successfulOperation = jest.fn().mockResolvedValue("success");

      const result = await retry(successfulOperation);

      expect(result).toBe("success");
      expect(successfulOperation).toHaveBeenCalledTimes(1);
    });

    it("should return different types of values", async () => {
      const numberOperation = jest.fn().mockResolvedValue(42);
      const objectOperation = jest.fn().mockResolvedValue({ key: "value" });
      const arrayOperation = jest.fn().mockResolvedValue([1, 2, 3]);

      expect(await retry(numberOperation)).toBe(42);
      expect(await retry(objectOperation)).toEqual({ key: "value" });
      expect(await retry(arrayOperation)).toEqual([1, 2, 3]);
    });
  });

  describe("retry logic", () => {
    it("should retry the specified number of times before failing", async () => {
      const failingOperation = jest
        .fn()
        .mockRejectedValue(new Error("test error"));

      await expect(
        retry(failingOperation, { retries: 3, delay: 1 }),
      ).rejects.toThrow("test error");
      expect(failingOperation).toHaveBeenCalledTimes(4); // 1 initial + 3 retries
    });

    it("should succeed on the second attempt", async () => {
      const operation = jest
        .fn()
        .mockRejectedValueOnce(new Error("first failure"))
        .mockResolvedValue("success");

      const result = await retry(operation, { retries: 3, delay: 1 });

      expect(result).toBe("success");
      expect(operation).toHaveBeenCalledTimes(2);
    });

    it("should succeed on the last retry attempt", async () => {
      const operation = jest
        .fn()
        .mockRejectedValueOnce(new Error("first failure"))
        .mockRejectedValueOnce(new Error("second failure"))
        .mockRejectedValueOnce(new Error("third failure"))
        .mockResolvedValue("success");

      const result = await retry(operation, { retries: 3, delay: 1 });

      expect(result).toBe("success");
      expect(operation).toHaveBeenCalledTimes(4);
    });

    it("should handle zero retries", async () => {
      const failingOperation = jest
        .fn()
        .mockRejectedValue(new Error("test error"));

      await expect(retry(failingOperation, { retries: 0 })).rejects.toThrow(
        "test error",
      );
      expect(failingOperation).toHaveBeenCalledTimes(1);
    });

    it("should handle one retry", async () => {
      const failingOperation = jest
        .fn()
        .mockRejectedValue(new Error("test error"));

      await expect(
        retry(failingOperation, { retries: 1, delay: 1 }),
      ).rejects.toThrow("test error");
      expect(failingOperation).toHaveBeenCalledTimes(2); // 1 initial + 1 retry
    });
  });

  describe("delay functionality", () => {
    it("should respect delay configuration", async () => {
      const operation = jest
        .fn()
        .mockRejectedValueOnce(new Error("failure"))
        .mockResolvedValue("success");

      const result = await retry(operation, { retries: 1, delay: 10 });

      expect(result).toBe("success");
      expect(operation).toHaveBeenCalledTimes(2);
    });
  });

  describe("shouldRetry functionality", () => {
    it("should not retry when shouldRetry returns false", async () => {
      const operation = jest.fn().mockRejectedValue(new Error("test error"));
      const shouldRetry = jest.fn().mockReturnValue(false);

      await expect(
        retry(operation, { retries: 3, shouldRetry }),
      ).rejects.toThrow("test error");
      expect(operation).toHaveBeenCalledTimes(1);
      expect(shouldRetry).toHaveBeenCalledWith(expect.any(Error));
    });

    it("should retry only for specific error types", async () => {
      const networkError = new Error("Network error");
      const validationError = new Error("Validation error");

      const operation = jest
        .fn()
        .mockRejectedValueOnce(networkError)
        .mockRejectedValueOnce(validationError);

      const shouldRetry = jest.fn(
        (error: unknown) =>
          error instanceof Error && error.message.includes("Network"),
      );

      await expect(
        retry(operation, { retries: 3, shouldRetry, delay: 1 }),
      ).rejects.toThrow("Validation error");
      expect(operation).toHaveBeenCalledTimes(2);
      expect(shouldRetry).toHaveBeenCalledTimes(2);
    });

    it("should pass the error to shouldRetry function", async () => {
      const testError = new TypeError("Type error");
      const operation = jest.fn().mockRejectedValue(testError);
      const shouldRetry = jest.fn().mockReturnValue(false);

      await expect(retry(operation, { shouldRetry })).rejects.toThrow(
        testError,
      );
      expect(shouldRetry).toHaveBeenCalledWith(testError);
    });

    it("should use default shouldRetry (always true) when not provided", async () => {
      const operation = jest.fn().mockRejectedValue(new Error("test error"));

      await expect(retry(operation, { retries: 2, delay: 1 })).rejects.toThrow(
        "test error",
      );
      expect(operation).toHaveBeenCalledTimes(3); // 1 initial + 2 retries
    });
  });

  describe("error types", () => {
    it("should handle different error types", async () => {
      const typeError = new TypeError("Type error");

      await expect(
        retry(() => Promise.reject(typeError), { retries: 0 }),
      ).rejects.toThrow(typeError);
    });

    it("should handle non-Error thrown values", async () => {
      const operation = jest.fn().mockRejectedValue("string error");

      await expect(retry(operation, { retries: 0 })).rejects.toBe(
        "string error",
      );
    });
  });

  describe("default options", () => {
    it("should use default values when options not provided", async () => {
      const operation = jest.fn().mockRejectedValue(new Error("test error"));

      await expect(retry(operation, { delay: 1 })).rejects.toThrow(
        "test error",
      );
      expect(operation).toHaveBeenCalledTimes(4); // 1 initial + 3 retries (default)
    });

    it("should allow partial options", async () => {
      const operation = jest.fn().mockRejectedValue(new Error("test error"));

      await expect(retry(operation, { retries: 1 })).rejects.toThrow(
        "test error",
      );
      expect(operation).toHaveBeenCalledTimes(2); // 1 initial + 1 retry
    });
  });

  describe("complex scenarios", () => {
    it("should handle mixed success and failure patterns", async () => {
      const operation = jest
        .fn()
        .mockRejectedValueOnce(new Error("temp failure"))
        .mockResolvedValueOnce("temp success");

      const result = await retry(operation, { retries: 3, delay: 1 });
      expect(result).toBe("temp success");
      expect(operation).toHaveBeenCalledTimes(2);
    });
  });
});

import {
  errorFunction,
  safeExecute,
  successFunction,
} from "@/Error/safeExecute";

describe("errorFunction", () => {
  it("should create an error result", () => {
    const result = errorFunction(new Error("test"));
    expect(result.type).toBe("error");
    expect(result.error).toBeInstanceOf(Error);
  });
});

describe("successFunction", () => {
  it("should create a success result", () => {
    const result = successFunction(42);
    expect(result.type).toBe("success");
    expect(result.value).toBe(42);
  });
});

describe("safeExecute function", () => {
  describe("successful operations", () => {
    it("should return success type with string value", () => {
      const successfulOperation = () => "test";
      const result = safeExecute(successfulOperation);

      expect(result).toEqual({
        type: "success",
        value: "test",
      });
    });

    it("should return success type with number value", () => {
      const successfulOperation = () => 42;
      const result = safeExecute(successfulOperation);

      expect(result).toEqual({
        type: "success",
        value: 42,
      });
    });

    it("should return success type with object value", () => {
      const testObject = { key: "value" };
      const successfulOperation = () => testObject;
      const result = safeExecute(successfulOperation);

      expect(result).toEqual({
        type: "success",
        value: testObject,
      });
    });
  });

  describe("error handling", () => {
    it("should return error type with Error instance", () => {
      const errorOperation = () => {
        throw new Error("test error");
      };
      const result = safeExecute(errorOperation);

      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBeInstanceOf(Error);
        expect(result.error.message).toBe("test error");
      }
    });

    it("should return error type with custom error message", () => {
      const customError = new TypeError("custom type error");
      const errorOperation = () => {
        throw customError;
      };
      const result = safeExecute(errorOperation);

      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBe(customError);
        expect(result.error.message).toBe("custom type error");
      }
    });

    it("should handle non-Error thrown values", () => {
      const errorOperation = () => {
        // biome-ignore lint/style/useThrowOnlyError: ignore
        throw "string error";
      };
      const result = safeExecute(errorOperation);

      expect(result.type).toBe("error");
      if (result.type === "error") {
        expect(result.error).toBe("string error");
      }
    });
  });
});

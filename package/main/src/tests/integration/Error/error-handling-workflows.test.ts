import { safeExecute } from "@/Error/safeExecute";
import { calculator } from "@/Math/calculator";
import { parseJson } from "@/Tool/parseJson";
import { pipe } from "@/Tool/pipe";
import { string } from "@/Validate/string";
import { validateEmail } from "@/Validate/string/validateEmail";

/**
 * Integration tests for Error handling with other modules
 *
 * Tests the interaction between error handling and other utilities:
 * - Safe execution with mathematical operations
 * - Error handling in data parsing pipelines
 * - Validation with error recovery
 */
describe("Integration test for error handling workflows", () => {
  it("should handle calculator errors safely", () => {
    const validExpression = "10 + 5";
    const invalidExpression = "invalid expression";

    const validResult = safeExecute(() => calculator(validExpression));
    const invalidResult = safeExecute(() => {
      if (invalidExpression === "invalid expression") {
        throw new Error("Invalid expression");
      }
      return calculator(invalidExpression);
    });

    expect(validResult.type).toBe("success");
    if (validResult.type === "success") {
      expect(validResult.value).toBe("15");
    }

    expect(invalidResult.type).toBe("error");
  });

  it("should handle JSON parsing errors in data processing pipelines", () => {
    const jsonStrings = [
      '{"name": "Alice", "age": 30}',
      "invalid json",
      '{"name": "Bob"}',
      '{"age": "not a number"}',
    ];

    const processJsonSafely = (jsonStr: string) => {
      const parseResult = safeExecute(() =>
        parseJson<{ name?: string; age?: number }>(jsonStr),
      );

      if (parseResult.type === "error") {
        return { error: "Invalid JSON", data: null };
      }

      const data = parseResult.value;
      return {
        error: null,
        data: {
          name: data.name || "Unknown",
          age: typeof data.age === "number" ? data.age : 0,
          valid: !!(data.name && typeof data.age === "number"),
        },
      };
    };

    const results = jsonStrings.map(processJsonSafely);

    expect(results[0].data?.valid).toBe(true);
    expect(results[1].error).toBe("Invalid JSON");
    expect(results[2].data?.valid).toBe(false);
    expect(results[3].data?.valid).toBe(false);
  });

  it("should create error-resilient data transformation pipelines", () => {
    const userData = [
      '{"email": "alice@example.com", "score": "85"}',
      '{"email": "invalid-email", "score": "90"}',
      "invalid json",
      '{"email": "bob@test.com", "score": "not-a-number"}',
    ];

    const processUserData = (jsonStr: string) => {
      const parseResult = safeExecute(() =>
        parseJson<{ email: string; score: string }>(jsonStr),
      );

      if (parseResult.type === "error") {
        return { success: false, error: "Parse error", data: null };
      }

      const { email: userEmail, score: scoreStr } = parseResult.value;

      const emailValidation = string([validateEmail()])(userEmail);
      const scoreResult = safeExecute(() => {
        const num = Number.parseFloat(scoreStr);
        if (Number.isNaN(num)) {
          throw new Error("Invalid score");
        }
        return num;
      });

      return {
        success: emailValidation.validate && scoreResult.type === "success",
        error: emailValidation.validate
          ? scoreResult.type === "error"
            ? "Invalid score"
            : null
          : "Invalid email",
        data: {
          email: userEmail,
          score: scoreResult.type === "success" ? scoreResult.value : 0,
          valid: emailValidation.validate && scoreResult.type === "success",
        },
      };
    };

    const results = userData.map(processUserData);

    expect(results[0].success).toBe(true);
    expect(results[0].data?.score).toBe(85);

    expect(results[1].success).toBe(false);
    expect(results[1].error).toBe("Invalid email");

    expect(results[2].success).toBe(false);
    expect(results[2].error).toBe("Parse error");

    expect(results[3].success).toBe(false);
    expect(results[3].error).toBe("Invalid score");
  });

  it("should handle mathematical operations with fallback values", () => {
    const calculateWithFallback = (expression: string, fallback = 0) => {
      const calcResult = safeExecute(() => {
        if (expression === "invalid") {
          throw new Error("Invalid expression");
        }
        return Number(calculator(expression));
      });

      if (calcResult.type === "error") {
        return { value: fallback, fromFallback: true };
      }

      const value = calcResult.value;
      return {
        value: Number.isNaN(value) ? fallback : value,
        fromFallback: Number.isNaN(value),
      };
    };

    const expressions = [
      { expr: "10 + 5", expected: 15 },
      { expr: "invalid", expected: 0 },
      { expr: "20 / 4", expected: 5 },
    ];

    const results = expressions.map(({ expr }) => calculateWithFallback(expr));

    expect(results[0].value).toBe(15);
    expect(results[0].fromFallback).toBe(false);

    expect(results[1].value).toBe(0);
    expect(results[1].fromFallback).toBe(true);

    expect(results[2].value).toBe(5);
    expect(results[2].fromFallback).toBe(false);
  });

  it("should chain safe operations in complex workflows", () => {
    const processComplexData = (input: string) => {
      return pipe(input)
        .map((jsonStr) => {
          const result = safeExecute(() =>
            parseJson<{ operation: string; values: number[] }>(jsonStr),
          );
          return result.type === "success" ? result.value : null;
        })
        .map((data) => {
          if (!data) {
            return null;
          }

          const { operation, values } = data;
          if (!Array.isArray(values) || values.length === 0) {
            return null;
          }

          if (operation === "sum") {
            return values.reduce((a, b) => a + b, 0);
          }
          if (operation === "product") {
            return values.reduce((a, b) => a * b, 1);
          }

          return null;
        })
        .end();
    };

    const testInputs = [
      '{"operation": "sum", "values": [1, 2, 3, 4]}',
      '{"operation": "product", "values": [2, 3, 4]}',
      "invalid json",
      '{"operation": "invalid", "values": [1, 2]}',
      '{"operation": "sum", "values": []}',
    ];

    const results = testInputs.map(processComplexData);

    expect(results[0]).toBe(10);
    expect(results[1]).toBe(24);
    expect(results[2]).toBe(null);
    expect(results[3]).toBe(null);
    expect(results[4]).toBe(null);
  });

  it("should handle validation errors gracefully in data processing", () => {
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    const validateAndProcess = (userData: any[]) => {
      return userData.map((user) => {
        const parseResult = safeExecute(() => {
          if (typeof user !== "object" || !user) {
            throw new Error("Invalid user object");
          }
          return user;
        });

        if (parseResult.type === "error") {
          return { valid: false, error: "Invalid object", data: null };
        }

        const { email: userEmail, name, age } = parseResult.value;

        const emailValidation = string([validateEmail()])(userEmail || "");
        const ageResult = safeExecute(() => {
          const ageNum = Number(age);
          if (Number.isNaN(ageNum) || ageNum < 0 || ageNum > 150) {
            throw new Error("Invalid age");
          }
          return ageNum;
        });

        return {
          valid: emailValidation.validate && ageResult.type === "success",
          error: emailValidation.validate
            ? ageResult.type === "error"
              ? "Invalid age"
              : null
            : "Invalid email",
          data: {
            name: name || "Unknown",
            email: userEmail || "",
            age: ageResult.type === "success" ? ageResult.value : 0,
          },
        };
      });
    };

    const testData = [
      { email: "alice@example.com", name: "Alice", age: 30 },
      { email: "invalid-email", name: "Bob", age: 25 },
      { email: "charlie@test.com", name: "Charlie", age: "invalid" },
      null,
      { email: "diana@example.com", name: "Diana", age: -5 },
    ];

    const results = validateAndProcess(testData);

    expect(results[0].valid).toBe(true);
    expect(results[1].valid).toBe(false);
    expect(results[1].error).toBe("Invalid email");
    expect(results[2].valid).toBe(false);
    expect(results[2].error).toBe("Invalid age");
    expect(results[3].valid).toBe(false);
    expect(results[3].error).toBe("Invalid object");
    expect(results[4].valid).toBe(false);
    expect(results[4].error).toBe("Invalid age");
  });

  it("should provide error recovery mechanisms", () => {
    // biome-ignore lint/suspicious/noExplicitAny: ignore
    const processWithRetry = (operation: () => any, maxRetries = 2) => {
      let attempts = 0;
      let lastError: Error | null = null;

      while (attempts <= maxRetries) {
        const result = safeExecute(operation);

        if (result.type === "success") {
          return { success: true, value: result.value, attempts: attempts + 1 };
        }

        lastError = result.error as Error;
        attempts++;
      }

      return {
        success: false,
        error: lastError?.message || "Unknown error",
        attempts,
      };
    };

    let callCount = 0;
    const flakyOperation = () => {
      callCount++;
      if (callCount < 3) {
        throw new Error(`Attempt ${callCount} failed`);
      }
      return "15";
    };

    const result = processWithRetry(flakyOperation, 3);

    expect(result.success).toBe(true);
    expect(result.value).toBe("15");
    expect(result.attempts).toBe(3);
  });
});

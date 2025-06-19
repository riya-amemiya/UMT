import { curry } from "@/Function/curry";
import { pipe } from "@/Tool/pipe";
import { createPipeline } from "@/Tool/createPipeline";
import { parseJson } from "@/Tool/parseJson";

/**
 * Integration tests for Function composition with Tool utilities
 *
 * Tests the interaction between function composition utilities:
 * - Currying functions with pipe operations
 * - Complex data transformation pipelines
 * - Functional programming patterns
 */
describe("Integration test for function composition", () => {
  it("should combine curry with pipe for data transformations", () => {
    const add = (a: number, b: number) => a + b;
    const multiply = (a: number, b: number) => a * b;
    // biome-ignore lint/suspicious/noShadowRestrictedNames: ignore
    const toString = (n: number) => n.toString();

    const curriedAdd = curry(add);
    const curriedMultiply = curry(multiply);

    const result = pipe(5)
      .map(curriedAdd(3))
      .map(curriedMultiply(2))
      .map(toString)
      .end();

    expect(result).toBe("16");
  });

  it("should create complex data processing pipelines", () => {
    const jsonData =
      '{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}';

    const getUsers = (data: { users: Array<{ name: string; age: number }> }) =>
      data.users;
    const filterAdults = (users: Array<{ name: string; age: number }>) =>
      users.filter((u) => u.age >= 25);
    const mapNames = (users: Array<{ name: string; age: number }>) =>
      users.map((u) => u.name);
    const joinNames = (names: string[]) => names.join(", ");

    const result = pipe(jsonData)
      .map(parseJson<{ users: Array<{ name: string; age: number }> }>)
      .map(getUsers)
      .map(filterAdults)
      .map(mapNames)
      .map(joinNames)
      .end();

    expect(result).toBe("Alice, Bob");
  });

  it("should handle curried functions in pipeline operations", () => {
    const calculate = (operation: string, a: number, b: number) => {
      switch (operation) {
        case "add":
          return a + b;
        case "multiply":
          return a * b;
        case "divide":
          return a / b;
        default:
          return 0;
      }
    };

    const curriedCalculate = curry(calculate);
    const addFive = curriedCalculate("add")(5);
    const multiplyByThree = curriedCalculate("multiply")(3);
    const divideByTwo = curriedCalculate("divide");

    const result = createPipeline(10)(addFive)(multiplyByThree)((n) =>
      divideByTwo(n)(2),
    )();

    expect(result).toBe(22.5);
  });

  it("should compose mathematical operations with data parsing", () => {
    const mathExpressions = ['{"value": 10}', '{"value": 20}', '{"value": 30}'];

    const parseValue = (json: string) =>
      parseJson<{ value: number }>(json).value;
    const square = (n: number) => n * n;
    const halve = (n: number) => n / 2;

    const processExpression = (expr: string) =>
      pipe(expr).map(parseValue).map(square).map(halve).end();

    const results = mathExpressions.map(processExpression);
    expect(results).toEqual([50, 200, 450]);
  });

  it("should handle complex nested function compositions", () => {
    interface TestObject {
      id: number;
      name: string;
    }

    interface ProcessedObject extends TestObject {
      processed: boolean;
      timestamp: number;
    }

    const transformObject = (obj: TestObject): ProcessedObject => ({
      ...obj,
      processed: true,
      timestamp: Date.now(),
    });

    const serialize = (obj: ProcessedObject): string => JSON.stringify(obj);
    const deserialize = (json: string): ProcessedObject =>
      parseJson<ProcessedObject>(json);

    const roundTripTransform = createPipeline({ id: 1, name: "test" })(
      transformObject,
    )(serialize)(deserialize)();

    expect(roundTripTransform.id).toBe(1);
    expect(roundTripTransform.name).toBe("test");
    expect(roundTripTransform.processed).toBe(true);
    expect(typeof roundTripTransform.timestamp).toBe("number");
  });

  it("should combine currying with error-safe operations", () => {
    const safeDivide = (a: number, b: number) => (b === 0 ? null : a / b);
    const safeParseNumber = (str: string) => {
      const num = Number.parseFloat(str);
      return Number.isNaN(num) ? null : num;
    };

    const curriedSafeDivide = curry(safeDivide);
    const divideBy = (divisor: number) => (dividend: number) =>
      curriedSafeDivide(dividend)(divisor);

    const testCases = ["10", "20", "invalid", "0"];

    const results = testCases.map((input) =>
      pipe(input)
        .map(safeParseNumber)
        .map((num) => (num !== null ? divideBy(2)(num) : null))
        .end(),
    );

    expect(results).toEqual([5, 10, null, 0]);
  });

  it("should create reusable transformation pipelines", () => {
    const normalizeString = (str: string) => str.trim().toLowerCase();
    const removeSpaces = (str: string) => str.replace(/\s+/g, "");
    const addPrefix = curry(
      (prefix: string, str: string) => `${prefix}_${str}`,
    );

    const processString = (input: string) =>
      createPipeline(input)(normalizeString)(removeSpaces)(addPrefix("id"))();

    const testInputs = ["  Hello World  ", "Test String", "Another Example"];
    const identifiers = testInputs.map(processString);

    expect(identifiers).toEqual([
      "id_helloworld",
      "id_teststring",
      "id_anotherexample",
    ]);
  });

  it("should handle array transformations with functional composition", () => {
    const numbers = [1, 2, 3, 4, 5];

    const double = (n: number) => n * 2;
    const isEven = (n: number) => n % 2 === 0;
    const sum = (arr: number[]) => arr.reduce((a, b) => a + b, 0);

    const processArray = pipe(numbers)
      .map((arr) => arr.map(double))
      .map((arr) => arr.filter(isEven))
      .map(sum)
      .end();

    expect(processArray).toBe(30);
  });
});

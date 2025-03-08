import { keyBy } from "@/Object/keyBy";
import _ from "lodash";

describe("keyBy", () => {
  it("should create an object using property name as key", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "b2", name: "Bob" },
    ];
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should generate keys using a custom function", () => {
    const input = [
      { dir: "left", code: 97 },
      { dir: "right", code: 100 },
    ];

    const result = {
      a: { dir: "left", code: 97 },
      d: { dir: "right", code: 100 },
    };
    const output = keyBy(input, (o) => String.fromCharCode(o.code));
    const lodashOutput = _.keyBy(input, (o) => String.fromCharCode(o.code));

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should return an empty object for an empty array", () => {
    const result = {};
    const output = keyBy([], "id");
    const lodashOutput = _.keyBy([], "id");
    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should use later values when there are duplicate keys", () => {
    const input = [
      { id: "a1", name: "Alice" },
      { id: "a1", name: "Alex" },
    ];
    const result = {
      a1: { id: "a1", name: "Alex" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });

  it("should accept an object as input", () => {
    const input = {
      first: { id: "a1", name: "Alice" },
      second: { id: "b2", name: "Bob" },
    };
    const result = {
      a1: { id: "a1", name: "Alice" },
      b2: { id: "b2", name: "Bob" },
    };

    const output = keyBy(input, "id");
    const lodashOutput = _.keyBy(input, "id");

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });
  it("should act as identity function when iteratee is not specified", () => {
    const input = ["a", "b"];
    const result = {
      a: "a",
      b: "b",
    };
    const output = keyBy(input);
    const lodashOutput = _.keyBy(input);

    expect(output).toEqual(result);
    expect(lodashOutput).toEqual(result);
  });
});

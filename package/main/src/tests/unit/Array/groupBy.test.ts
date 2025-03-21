import { groupBy } from "@/Array/groupBy";

describe("groupBy", () => {
  it("should group numbers into odd and even", () => {
    const array = [1, 2, 3, 4, 5];
    const result = groupBy(array, (num) => (num % 2 === 0 ? "even" : "odd"));
    expect(result).toEqual({ odd: [1, 3, 5], even: [2, 4] });
  });

  it("should group strings based on their length", () => {
    const array = ["one", "two", "three", "four", "five"];
    const result = groupBy(array, (str) => str.length.toString());
    expect(result).toEqual({
      "3": ["one", "two"],
      "4": ["four", "five"],
      "5": ["three"],
    });
  });

  it("should group boolean values", () => {
    const array = [true, false, true, false, true];
    const result = groupBy(array, (bool) => bool.toString());
    expect(result).toEqual({ true: [true, true, true], false: [false, false] });
  });

  it("should group objects based on their properties", () => {
    const array: {
      type: "fruit" | "vegetable";
      name: string;
    }[] = [
      { type: "fruit", name: "apple" },
      { type: "vegetable", name: "carrot" },
      { type: "fruit", name: "banana" },
      { type: "vegetable", name: "lettuce" },
    ];
    const result = groupBy(array, (item) => item.type);
    expect(result).toEqual({
      fruit: [
        { type: "fruit", name: "apple" },
        { type: "fruit", name: "banana" },
      ],
      vegetable: [
        { type: "vegetable", name: "carrot" },
        { type: "vegetable", name: "lettuce" },
      ],
    });
  });

  it("should group all elements under undefined key when iterator returns undefined", () => {
    const array = [1, 2, 3, 4, 5];
    // @ts-ignore
    const result = groupBy(array, () => undefined);
    expect(result).toEqual({ undefined: [1, 2, 3, 4, 5] });
  });

  it("should return an empty object for an empty array", () => {
    const array: number[] = [];
    const result = groupBy(array, (num) => (num % 2 === 0 ? "even" : "odd"));
    expect(result).toEqual({});
  });
});

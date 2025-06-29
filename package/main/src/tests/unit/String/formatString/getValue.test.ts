import { getValue } from "@/String/formatString/getValue";

describe("getValue function", () => {
  describe("simple property access", () => {
    test("should retrieve simple string properties", () => {
      const obj = { name: "Alice", age: 30 };

      expect(getValue(obj, "name")).toBe("Alice");
      expect(getValue(obj, "age")).toBe(30);
    });

    test("should return undefined for non-existent properties", () => {
      const obj = { name: "Alice" };

      expect(getValue(obj, "nonexistent")).toBeUndefined();
      expect(getValue(obj, "missing")).toBeUndefined();
    });

    test("should handle various data types", () => {
      const obj = {
        string: "text",
        number: 42,
        boolean: true,
        nullValue: null,
        undefinedValue: undefined,
        zero: 0,
        emptyString: "",
      };

      expect(getValue(obj, "string")).toBe("text");
      expect(getValue(obj, "number")).toBe(42);
      expect(getValue(obj, "boolean")).toBe(true);
      expect(getValue(obj, "nullValue")).toBeNull();
      expect(getValue(obj, "undefinedValue")).toBeUndefined();
      expect(getValue(obj, "zero")).toBe(0);
      expect(getValue(obj, "emptyString")).toBe("");
    });
  });

  describe("nested property access", () => {
    test("should retrieve nested properties", () => {
      const obj = {
        user: {
          name: "Bob",
          profile: {
            age: 25,
            location: "Tokyo",
          },
        },
      };

      expect(getValue(obj, "user.name")).toBe("Bob");
      expect(getValue(obj, "user.profile.age")).toBe(25);
      expect(getValue(obj, "user.profile.location")).toBe("Tokyo");
    });

    test("should return undefined for broken nested paths", () => {
      const obj = {
        user: {
          name: "Bob",
        },
      };

      expect(getValue(obj, "user.nonexistent")).toBeUndefined();
      expect(getValue(obj, "user.profile.age")).toBeUndefined();
      expect(getValue(obj, "nonexistent.property")).toBeUndefined();
    });

    test("should handle deep nesting", () => {
      const obj = {
        level1: {
          level2: {
            level3: {
              level4: {
                value: "deep",
              },
            },
          },
        },
      };

      expect(getValue(obj, "level1.level2.level3.level4.value")).toBe("deep");
    });
  });

  describe("array access", () => {
    test("should access array elements with positive indices", () => {
      const obj = {
        items: ["A", "B", "C", "D"],
      };

      expect(getValue(obj, "items[0]")).toBe("A");
      expect(getValue(obj, "items[1]")).toBe("B");
      expect(getValue(obj, "items[2]")).toBe("C");
      expect(getValue(obj, "items[3]")).toBe("D");
    });

    test("should access array elements with negative indices", () => {
      const obj = {
        items: ["A", "B", "C", "D"],
      };

      expect(getValue(obj, "items[-1]")).toBe("D");
      expect(getValue(obj, "items[-2]")).toBe("C");
      expect(getValue(obj, "items[-3]")).toBe("B");
      expect(getValue(obj, "items[-4]")).toBe("A");
    });

    test("should return undefined for out-of-bounds indices", () => {
      const obj = {
        items: ["A", "B"],
      };

      expect(getValue(obj, "items[5]")).toBeUndefined();
      expect(getValue(obj, "items[-5]")).toBeUndefined();
    });

    test("should handle empty arrays", () => {
      const obj = {
        items: [],
      };

      expect(getValue(obj, "items[0]")).toBeUndefined();
      expect(getValue(obj, "items[-1]")).toBeUndefined();
    });
  });

  describe("complex nested access", () => {
    test("should handle arrays of objects", () => {
      const obj = {
        users: [
          { name: "Alice", age: 30 },
          { name: "Bob", age: 25 },
          { name: "Charlie", age: 35 },
        ],
      };

      expect(getValue(obj, "users[0].name")).toBe("Alice");
      expect(getValue(obj, "users[1].age")).toBe(25);
      expect(getValue(obj, "users[-1].name")).toBe("Charlie");
    });

    test("should handle nested arrays", () => {
      const obj = {
        matrix: [
          [1, 2, 3],
          [4, 5, 6],
          [7, 8, 9],
        ],
      };

      // Note: getValue doesn't support nested array notation like matrix[0][1]
      expect(getValue(obj, "matrix[0]")).toEqual([1, 2, 3]);
      expect(getValue(obj, "matrix[1]")).toEqual([4, 5, 6]);
      expect(getValue(obj, "matrix[-1]")).toEqual([7, 8, 9]);
    });

    test("should handle objects containing arrays containing objects", () => {
      const obj = {
        data: {
          categories: [
            {
              name: "Technology",
              items: [
                { title: "Laptop", price: 1000 },
                { title: "Phone", price: 500 },
              ],
            },
            {
              name: "Books",
              items: [{ title: "Novel", price: 20 }],
            },
          ],
        },
      };

      expect(getValue(obj, "data.categories[0].name")).toBe("Technology");
      expect(getValue(obj, "data.categories[0].items[1].title")).toBe("Phone");
      expect(getValue(obj, "data.categories[-1].items[0].price")).toBe(20);
    });
  });

  describe("edge cases", () => {
    test("should handle null and undefined objects", () => {
      expect(getValue(null, "property")).toBeUndefined();
      expect(getValue(undefined, "property")).toBeUndefined();
    });

    test("should handle primitive values as objects", () => {
      expect(getValue(42, "property")).toBeUndefined();
      expect(getValue("string", "property")).toBeUndefined();
      expect(getValue(true, "property")).toBeUndefined();
    });

    test("should handle empty path", () => {
      const obj = { name: "Alice" };
      expect(getValue(obj, "")).toBeUndefined();
    });

    test("should handle array access on non-arrays", () => {
      const obj = {
        notAnArray: "string",
      };

      expect(getValue(obj, "notAnArray[0]")).toBeUndefined();
    });

    test("should handle complex array notation", () => {
      const obj = {
        "complex[key]": "value",
        items: ["A", "B"],
      };

      // Should parse "items[0]" correctly
      expect(getValue(obj, "items[0]")).toBe("A");

      // Complex keys with brackets in names are handled differently
      expect(getValue(obj, "complex[key]")).toBe("value");
    });
  });

  describe("special path formats", () => {
    test("should handle zero indices", () => {
      const obj = {
        items: ["zero", "one", "two"],
      };

      expect(getValue(obj, "items[0]")).toBe("zero");
      expect(getValue(obj, "items[-0]")).toBe("zero");
    });

    test("should handle array indices in the middle of paths", () => {
      const obj = {
        groups: [
          {
            name: "Group1",
            subGroups: [
              { name: "SubGroup1", value: "test1" },
              { name: "SubGroup2", value: "test2" },
            ],
          },
        ],
      };

      expect(getValue(obj, "groups[0].subGroups[1].value")).toBe("test2");
    });

    test("should handle single array access with nested objects", () => {
      const obj = {
        grid: [
          [{ value: "cell_0_0" }, { value: "cell_0_1" }],
          [{ value: "cell_1_0" }, { value: "cell_1_1" }],
        ],
      };

      // getValue doesn't support grid[0][1] syntax
      expect(getValue(obj, "grid[1]")).toEqual([
        { value: "cell_1_0" },
        { value: "cell_1_1" },
      ]);
    });
  });

  describe("data type preservation", () => {
    test("should preserve original data types", () => {
      const obj = {
        numbers: [0, 1, -1, 3.14],
        booleans: [true, false],
        objects: [{ key: "value" }],
        mixed: [null, undefined, "", 0, false],
      };

      expect(getValue(obj, "numbers[0]")).toBe(0);
      expect(getValue(obj, "numbers[2]")).toBe(-1);
      expect(getValue(obj, "numbers[3]")).toBe(3.14);
      expect(getValue(obj, "booleans[0]")).toBe(true);
      expect(getValue(obj, "booleans[1]")).toBe(false);
      expect(getValue(obj, "objects[0]")).toEqual({ key: "value" });
      expect(getValue(obj, "mixed[0]")).toBeNull();
      expect(getValue(obj, "mixed[1]")).toBeUndefined();
      expect(getValue(obj, "mixed[2]")).toBe("");
      expect(getValue(obj, "mixed[3]")).toBe(0);
      expect(getValue(obj, "mixed[4]")).toBe(false);
    });
  });
});

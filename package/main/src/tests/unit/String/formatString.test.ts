import { formatString } from "@/String/formatString";

describe("formatString", () => {
  describe("Original functionality", () => {
    it("should replace placeholders with specified values", () => {
      const template = "Hello, {0}! It's {1} today.";
      const result = formatString(template, "World", "sunny");
      expect(result).toBe("Hello, World! It's sunny today.");
    });

    it("should not replace placeholders with undefined values", () => {
      const template = "Hello, {0}! How's {1}?";
      const result = formatString(template, "World");
      expect(result).toBe("Hello, World! How's {1}?");
    });

    it("should not replace placeholders with non-existent indices", () => {
      const template = "Hello, {0}! {2} is not available.";
      const result = formatString(template, "World", "sunny");
      expect(result).toBe("Hello, World! {2} is not available.");
    });

    it("should handle special characters in placeholder values", () => {
      const template = "Hello {0}! Special chars: {1}";
      const result = formatString(template, "🌍", "©®™%$#@!");
      expect(result).toBe("Hello 🌍! Special chars: ©®™%$#@!");
    });

    it("should handle empty strings as values", () => {
      const template = "Start{0}Middle{1}End";
      const result = formatString(template, "", "");
      expect(result).toBe("StartMiddleEnd");
    });

    it("should convert non-string values to strings", () => {
      const template = "Number: {0}, Object: {1}, Array: {2}";
      const object = { key: "value" };
      const result = formatString(template, 123, object, [1, 2, 3]);
      expect(result).toBe("Number: 123, Object: [object Object], Array: 1,2,3");
    });

    it("should handle repeated placeholders", () => {
      const template = "{0} {1} {0} {1} {0}";
      const result = formatString(template, "A", "B");
      expect(result).toBe("A B A B A");
    });

    it("should handle whitespace characters in values", () => {
      const template = "Space:{0}, Tab:{1}, Newline:{2}";
      const result = formatString(template, " ", "\t", "\n");
      expect(result).toBe("Space: , Tab:\t, Newline:\n");
    });
  });

  describe("Named placeholders", () => {
    it("should replace named placeholders with object values", () => {
      const template = "Hello, {name}! You are {age} years old.";
      const result = formatString(template, { name: "Alice", age: 25 });
      expect(result).toBe("Hello, Alice! You are 25 years old.");
    });

    it("should handle missing named placeholders", () => {
      const template = "Hello, {name}! How's {mood}?";
      const result = formatString(template, { name: "Bob" });
      expect(result).toBe("Hello, Bob! How's {mood}?");
    });
  });

  describe("Nested object access", () => {
    it("should access nested object properties", () => {
      const template = "User: {user.name}, Email: {user.email}";
      const result = formatString(template, {
        user: { name: "Charlie", email: "charlie@example.com" },
      });
      expect(result).toBe("User: Charlie, Email: charlie@example.com");
    });

    it("should handle deep nesting", () => {
      const template = "Address: {user.address.city}, {user.address.country}";
      const result = formatString(template, {
        user: {
          address: {
            city: "Tokyo",
            country: "Japan",
          },
        },
      });
      expect(result).toBe("Address: Tokyo, Japan");
    });

    it("should handle missing nested properties", () => {
      const template = "User: {user.name}, Phone: {user.phone}";
      const result = formatString(template, { user: { name: "Dave" } });
      expect(result).toBe("User: Dave, Phone: {user.phone}");
    });

    it("should handle null values in nested access", () => {
      const template = "Value: {data.nested}";
      const result = formatString(template, { data: null });
      expect(result).toBe("Value: {data.nested}");
    });

    it("should handle primitive values in nested access", () => {
      const template = "Value: {data.nested}";
      const result = formatString(template, { data: "string" });
      expect(result).toBe("Value: {data.nested}");
    });
  });

  describe("Array access", () => {
    it("should access array elements by positive index", () => {
      const template = "First: {items[0]}, Second: {items[1]}";
      const result = formatString(template, { items: ["A", "B", "C"] });
      expect(result).toBe("First: A, Second: B");
    });

    it("should access array elements by negative index", () => {
      const template = "Last: {items[-1]}, Second last: {items[-2]}";
      const result = formatString(template, { items: ["A", "B", "C"] });
      expect(result).toBe("Last: C, Second last: B");
    });

    it("should handle out of bounds array access", () => {
      const template = "Item: {items[10]}";
      const result = formatString(template, { items: ["A", "B"] });
      expect(result).toBe("Item: {items[10]}");
    });

    it("should handle nested array access", () => {
      const template = "First user: {users[0].name}";
      const result = formatString(template, {
        users: [{ name: "Alice" }, { name: "Bob" }],
      });
      expect(result).toBe("First user: Alice");
    });

    it("should handle array access on non-array values", () => {
      const template = "Value: {data[0]}";
      const result = formatString(template, { data: "not an array" });
      expect(result).toBe("Value: {data[0]}");
    });
  });

  describe("Default values", () => {
    it("should use default values for missing placeholders", () => {
      const template = "Name: {name|Unknown}, Age: {age|N/A}";
      const result = formatString(template, { age: 25 });
      expect(result).toBe("Name: Unknown, Age: 25");
    });

    it("should use actual values when available", () => {
      const template = "Name: {name|Unknown}, Age: {age|N/A}";
      const result = formatString(template, { name: "Eve", age: 30 });
      expect(result).toBe("Name: Eve, Age: 30");
    });

    it("should handle default values with null/undefined", () => {
      const template = "Value: {value|Default}";
      const result1 = formatString(template, { value: null });
      const result2 = formatString(template, { value: undefined });
      expect(result1).toBe("Value: Default");
      expect(result2).toBe("Value: Default");
    });
  });

  describe("Escape sequences", () => {
    it("should handle escaped braces", () => {
      const template = "Literal {{0}} and value {0}";
      const result = formatString(template, "test");
      expect(result).toBe("Literal {0} and value test");
    });

    it("should handle multiple escaped braces", () => {
      const template = "{{name}} is not {name}, but {{age}} is not {age}";
      const result = formatString(template, { name: "Alice", age: 25 });
      expect(result).toBe("{name} is not Alice, but {age} is not 25");
    });
  });

  describe("Formatters", () => {
    describe("Built-in formatters", () => {
      it("should format with upper formatter", () => {
        const template = "Name: {name:upper}";
        const result = formatString(template, { name: "alice" });
        expect(result).toBe("Name: ALICE");
      });

      it("should format with lower formatter", () => {
        const template = "Name: {name:lower}";
        const result = formatString(template, { name: "ALICE" });
        expect(result).toBe("Name: alice");
      });

      it("should format currency", () => {
        const template = "Price: {price:currency}";
        const result = formatString(template, { price: 1234.56 });
        expect(result).toMatch(/\$1,234\.56/);
      });

      it("should format currency with locale", () => {
        const template = "Price: {price:currency(ja-JP,JPY)}";
        const result = formatString(template, { price: 1234 });
        expect(result).toMatch(/￥1,234/);
      });

      it("should format dates", () => {
        const template = "Date: {date:date}";
        const date = new Date("2023-12-25");
        const result = formatString(template, { date });
        expect(result).toMatch(/Date: \d{1,2}\/\d{1,2}\/\d{4}/);
      });

      it("should format dates as ISO", () => {
        const template = "Date: {date:date(en-US,iso)}";
        const date = new Date("2023-12-25T10:30:00.000Z");
        const result = formatString(template, { date });
        expect(result).toBe("Date: 2023-12-25T10:30:00.000Z");
      });

      it("should format dates as time using date formatter", () => {
        const template = "Time: {date:date(en-US,time)}";
        const date = new Date("2023-12-25T10:30:00");
        const result = formatString(template, { date });
        expect(result).toMatch(/Time: \d{1,2}:\d{2}:\d{2}/);
      });

      it("should format time", () => {
        const template = "Time: {time:time}";
        const date = new Date("2023-12-25T10:30:00");
        const result = formatString(template, { time: date });
        expect(result).toMatch(/Time: \d{1,2}:\d{2}:\d{2}/);
      });

      it("should format numbers", () => {
        const template = "Number: {num:number(en-US,2,2)}";
        const result = formatString(template, { num: 123.456 });
        expect(result).toBe("Number: 123.46");
      });

      it("should handle plural formatting", () => {
        const template1 = "You have {count:plural(item,items)}";
        const template2 = "{count} {count:plural(item,items)}";

        const result1 = formatString(template1, { count: 1 });
        const result2 = formatString(template2, { count: 1 });
        const result3 = formatString(template2, { count: 5 });

        expect(result1).toBe("You have item");
        expect(result2).toBe("1 item");
        expect(result3).toBe("5 items");
      });

      it("should pad numbers", () => {
        const template = "ID: {id:pad(4,0)}";
        const result = formatString(template, { id: 42 });
        expect(result).toBe("ID: 0042");
      });

      it("should pad with custom character", () => {
        const template = "Value: {value:pad(5,#)}";
        const result = formatString(template, { value: "hi" });
        expect(result).toBe("Value: ###hi");
      });
    });

    describe("Custom formatters", () => {
      it("should use custom formatters", () => {
        const template = "Reversed: {text:reverse}";
        const options = {
          formatters: {
            reverse: (value: unknown) =>
              String(value).split("").reverse().join(""),
          },
        };
        const result = formatString(template, { text: "hello" }, options);
        expect(result).toBe("Reversed: olleh");
      });

      it("should override built-in formatters", () => {
        const template = "Custom upper: {text:upper}";
        const options = {
          formatters: {
            upper: (value: unknown) => `[${String(value).toUpperCase()}]`,
          },
        };
        const result = formatString(template, { text: "hello" }, options);
        expect(result).toBe("Custom upper: [HELLO]");
      });

      it("should handle custom formatters with arguments", () => {
        const template = "Repeat: {text:repeat(3)}";
        const options = {
          formatters: {
            repeat: (value: unknown, times: string) =>
              String(value).repeat(Number(times)),
          },
        };
        const result = formatString(template, { text: "hi" }, options);
        expect(result).toBe("Repeat: hihihi");
      });

      it("should handle invalid formatter syntax", () => {
        const template = "Value: {text:invalid-formatter-name!@#}";
        const result = formatString(template, { text: "hello" });
        expect(result).toBe("Value: hello");
      });

      it("should handle unknown formatter names", () => {
        const template = "Value: {text:nonExistentFormatter}";
        const result = formatString(template, { text: "hello" });
        expect(result).toBe("Value: hello");
      });

      it("should handle Date objects with formatters", () => {
        const template = "Created: {date:date}, Time: {date:time}";
        const date = new Date("2023-12-25T10:30:00");
        const result = formatString(template, { date });
        expect(result).toMatch(
          /Created: \d{1,2}\/\d{1,2}\/\d{4}, Time: \d{1,2}:\d{2}:\d{2}/,
        );
      });

      it("should handle number formatter with custom precision", () => {
        const template = "Value: {num:number(en-US,3,5)}";
        const result = formatString(template, { num: 123.456789 });
        expect(result).toBe("Value: 123.45679");
      });

      it("should handle number formatter with default precision", () => {
        const template = "Value: {num:number}";
        const result = formatString(template, { num: 123.456 });
        expect(result).toBe("Value: 123.456");
      });

      it("should handle pad formatter with default values", () => {
        const template = "Value: {num:pad}";
        const result = formatString(template, { num: 5 });
        expect(result).toBe("Value: 05");
      });

      it("should handle Date object directly in date formatter", () => {
        const template = "Date: {date:date}";
        const date = new Date("2023-12-25");
        const result = formatString(template, { date });
        expect(result).toMatch(/Date: \d{1,2}\/\d{1,2}\/\d{4}/);
      });

      it("should handle Date object directly in time formatter", () => {
        const template = "Time: {date:time}";
        const date = new Date("2023-12-25T10:30:00");
        const result = formatString(template, { date });
        expect(result).toMatch(/Time: \d{1,2}:\d{2}:\d{2}/);
      });

      it("should format string values as dates", () => {
        const template = "Date: {date:date}";
        const result = formatString(template, { date: "2023-12-25" });
        expect(result).toMatch(/Date: \d{1,2}\/\d{1,2}\/\d{4}/);
      });

      it("should format string values as time", () => {
        const template = "Time: {time:time}";
        const result = formatString(template, { time: "2023-12-25T10:30:00" });
        expect(result).toMatch(/Time: \d{1,2}:\d{2}:\d{2}/);
      });

      it("should handle number formatter with locale only", () => {
        const template = "Value: {num:number(ja-JP)}";
        const result = formatString(template, { num: 1234.5 });
        expect(result).toMatch(/1,234\.5|1 234,5|1\.234,5/);
      });
    });
  });

  describe("Complex combinations", () => {
    it("should combine all features", () => {
      const template =
        "Hello {{escaped}}, {user.name:upper|Unknown}! You have {items[-1]:pad(3,0)} {count:plural(item,items)} at {user.address.city|Home}.";
      const data = {
        user: {
          name: "alice",
          address: { city: "Tokyo" },
        },
        items: [1, 2, 42],
        count: 3,
      };
      const result = formatString(template, data);
      expect(result).toBe(
        "Hello {escaped}, ALICE! You have 042 items at Tokyo.",
      );
    });

    it("should handle mixed indexed and named-like patterns", () => {
      const template = "Index {0}, Named {name}, Default {missing|def}";
      const result = formatString(template, "first", "second");
      expect(result).toBe("Index first, Named {name}, Default def");
    });
  });

  describe("Edge cases", () => {
    it("should handle empty template", () => {
      const result = formatString("", { name: "test" });
      expect(result).toBe("");
    });

    it("should handle template with only escaped braces", () => {
      const result = formatString("{{}} {{test}}", { test: "value" });
      expect(result).toBe("{} {test}");
    });

    it("should handle complex nesting in arrays", () => {
      const template = "Nested: {data[0].items[1].name}";
      const data = {
        data: [
          {
            items: [{ name: "first" }, { name: "second" }],
          },
        ],
      };
      const result = formatString(template, data);
      expect(result).toBe("Nested: second");
    });

    it("should handle malformed placeholders gracefully", () => {
      const template = "Bad: {unclosed, Good: {name}";
      const result = formatString(template, { name: "Alice" });
      expect(result).toBe("Bad: {unclosed, Good: {name}");
    });

    it("should handle edge case with options object detection", () => {
      const template = "Value: {test}";
      const options = { formatters: {} };
      const result = formatString(template, { test: "hello" }, options);
      expect(result).toBe("Value: hello");
    });

    it("should handle Date objects in indexed mode", () => {
      const template = "Date: {0}";
      const date = new Date("2023-12-25");
      const result = formatString(template, date);
      expect(result).toMatch(/Date: /);
    });

    it("should handle array objects in indexed mode", () => {
      const template = "Array: {0}";
      const array = [1, 2, 3];
      const result = formatString(template, array);
      expect(result).toBe("Array: 1,2,3");
    });

    it("should work with single object argument", () => {
      const template = "{name}";
      const result = formatString(template, { name: "test" });
      expect(result).toBe("test");
    });

    it("should work with exactly two arguments", () => {
      const template = "{0} and {1}";
      const result = formatString(template, "first", "second");
      expect(result).toBe("first and second");
    });

    it("should detect mode with minimal arguments", () => {
      const template = "{name}";
      const result = formatString(template, { name: "test" });
      expect(result).toBe("test");
    });
  });
});

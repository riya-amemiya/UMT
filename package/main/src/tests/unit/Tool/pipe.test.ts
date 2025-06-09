import { Pipe, pipe } from "@/Tool/pipe";
import { isNumber, isString } from "@/Validate";

interface User {
  id: number;
  name: string;
  age: number;
}

interface Post {
  id: number;
  title: string;
  content: string;
  author: User;
}

describe("Pipe", () => {
  // Direct constructor test
  it("creates an instance with the constructor", () => {
    const pipeLine = new Pipe(42);
    expect(pipeLine).toBeInstanceOf(Pipe);
    expect(pipeLine.end()).toBe(42);
  });

  // Extension test
  it("can be extended with additional methods", () => {
    // Extended pipe class with additional functionality
    class ExtendedPipe<T> extends Pipe<T> {
      double(): ExtendedPipe<number> {
        const currentValue = this.end();
        if (typeof currentValue === "number") {
          return new ExtendedPipe(currentValue * 2);
        }
        throw new Error("Value is not a number");
      }

      prefix(str: string): ExtendedPipe<string> {
        return new ExtendedPipe(`${str}${this.end()}`);
      }
    }

    // Helper function for the extended pipe
    function extendedPipe<T>(value: T): ExtendedPipe<T> {
      return new ExtendedPipe(value);
    }

    // Test the extended functionality
    const result = extendedPipe(5)
      .double()
      .map((x) => x + 1)
      .end();
    expect(result).toBe(11);

    const strResult = extendedPipe("world")
      .prefix("hello ")
      .map((s) => s.toUpperCase())
      .end();
    expect(strResult).toBe("HELLO WORLD");
  });

  // Basic functionality tests
  it("returns initial value when calling end()", () => {
    const result = pipe(1).end();
    expect(result).toBe(1);
  });

  it("transforms value with map()", () => {
    const result = pipe(1)
      .map((x) => x + 1)
      .end();
    expect(result).toBe(2);
  });

  it("chains multiple map() operations", () => {
    const result = pipe(1)
      .map((x) => x + 1)
      .map((x) => x * 2)
      .map((x) => x - 1)
      .end();
    expect(result).toBe(3);
  });

  it("applies transformation when condition is true in when()", () => {
    const result = pipe(5)
      .when(
        (x) => x > 3,
        (x) => x * 2,
      )
      .end();
    expect(result).toBe(10);
  });

  it("skips transformation when condition is false in when()", () => {
    const result = pipe(2)
      .when(
        (x) => x > 3,
        (x) => x * 2,
      )
      .end();
    expect(result).toBe(2);
  });

  it("executes side effect and preserves original value with tap()", () => {
    let sideEffect = 0;
    const result = pipe(5)
      .tap((x) => {
        sideEffect = x;
      })
      .end();

    expect(result).toBe(5);
    expect(sideEffect).toBe(5);
  });

  it("correctly infers types", () => {
    const numberPipe = pipe(1);
    const stringPipe = numberPipe.map((x) => `${x}`);
    const result: string = stringPipe.end();
    expect(result).toBe("1");
  });

  it("handles complex type inference correctly", () => {
    const user: User = {
      id: 1,
      name: "John Doe",
      age: 30,
    };

    const post: Post = pipe(user)
      .map((user: User) => ({
        id: 1,
        title: "First Post",
        content: "Hello, world!",
        author: user,
      }))
      .end();

    expect(post).toEqual({
      id: 1,
      title: "First Post",
      content: "Hello, world!",
      author: {
        id: 1,
        name: "John Doe",
        age: 30,
      },
    });
  });

  it("correctly infers nested generic types", () => {
    const users: User[] = pipe<User[]>([])
      .map((users) => [...users, { id: 1, name: "John Doe", age: 30 }])
      .map((users) => [...users, { id: 2, name: "Jane Smith", age: 25 }])
      .end();

    expect(users).toEqual([
      { id: 1, name: "John Doe", age: 30 },
      { id: 2, name: "Jane Smith", age: 25 },
    ]);
  });

  it("processes null as initial value correctly", () => {
    const result = pipe(null).end();
    expect(result).toBeNull();
  });

  it("processes undefined as initial value correctly", () => {
    const result = pipe(undefined).end();
    expect(result).toBeUndefined();
  });

  it("processes empty strings correctly", () => {
    const result = pipe("")
      .map((x) => x + "test")
      .map((x) => x.toUpperCase())
      .end();
    expect(result).toBe("TEST");
  });

  it("processes numeric zero correctly", () => {
    const result = pipe(0)
      .map((x) => x + 1)
      .map((x) => x * 2)
      .end();
    expect(result).toBe(2);
  });

  it("processes boolean transformations correctly", () => {
    const result = pipe(true)
      .map((x) => !x)
      .map((x) => x.toString())
      .end();
    expect(result).toBe("false");
  });

  it("processes array transformations correctly", () => {
    const result = pipe([1, 2, 3, 4, 5])
      .map((arr) => arr.filter((x) => x % 2 === 0))
      .map((arr) => arr.map((x) => x * 2))
      .end();
    expect(result).toEqual([4, 8]);
  });

  it("processes complex object transformations correctly", () => {
    interface Data {
      count: number;
      items: string[];
    }

    const initial: Data = { count: 0, items: [] };
    const result = pipe(initial)
      .map((data) => ({ ...data, count: data.count + 1 }))
      .map((data) => ({
        ...data,
        items: [...data.items, `Item ${data.count}`],
      }))
      .end();

    expect(result).toEqual({
      count: 1,
      items: ["Item 1"],
    });
  });

  it("processes multiple type transformations in chain", () => {
    const result = pipe(123)
      .map((x) => x.toString())
      .map((x) => x.split(""))
      .map((arr) => arr.map(Number))
      .end();

    expect(result).toEqual([1, 2, 3]);
  });

  it("combines when() and map() for complex processing", () => {
    const result = pipe(5)
      .map((x) => x + 2)
      .when(
        (x) => x > 5,
        (x) => x * 2,
      )
      .when(
        (x) => x < 5,
        (x) => x - 1,
      )
      .map((x) => x + 1)
      .end();

    expect(result).toBe(15);
  });

  describe("filterStrict()", () => {
    interface ValidUser extends User {
      verified: boolean;
    }
    function isValidUser(user: User): user is ValidUser {
      return "verified" in user;
    }

    it("filters and narrows type using type predicate", () => {
      const result = pipe<unknown>(42)
        .filterStrict((x) => isNumber(x, false))
        .map((x) => x + 1)
        .end();
      expect(result).toBe(43);
    });

    it("throws error when filter condition is not met", () => {
      expect(() => {
        pipe<unknown>("not a number").filterStrict(isNumber).end();
      }).toThrow("Value did not match filter predicate");
    });

    it("filters complex types with interface predicates", () => {
      const user: ValidUser = {
        id: 1,
        name: "John",
        age: 30,
        verified: true,
      };

      const result = pipe<User>(user)
        .filterStrict(isValidUser)
        .map((u) => u.verified)
        .end();

      expect(result).toBe(true);
    });

    it("combines filterStrict with when and map operations", () => {
      const result = pipe<unknown>(5)
        .filterStrict((x) => isNumber(x, false))
        .map((x) => x + 2)
        .when(
          (x) => x > 5,
          (x) => x * 2,
        )
        .end();

      expect(result).toBe(14);
    });

    it("correctly handles type narrowing with string | number union types", () => {
      const stringResult = pipe<string | number>("hello")
        .filterStrict(isString)
        .map((x) => x + " world")
        .end();
      expect(stringResult).toBe("hello world");

      const numberResult = pipe<string | number>(123)
        .filterStrict((x) => isNumber(x, false))
        .map((x) => x * 2)
        .end();
      expect(numberResult).toBe(246);

      const whenResult = pipe<string | number>(5)
        .filterStrict((x) => isNumber(x, false))
        .when(
          (x) => x > 0,
          (x) => x * 2,
        )
        .end();
      expect(whenResult).toBe(10);
    });
  });

  describe("filterWithDefault()", () => {
    it("returns original value when predicate is true", () => {
      const result = pipe<unknown>(42)
        .filterWithDefault((x) => isNumber(x, false), 0)
        .map((x) => x + 1)
        .end();
      expect(result).toBe(43);
    });

    it("returns default value when predicate is false", () => {
      const result = pipe<unknown>("not a number")
        .filterWithDefault((x) => isNumber(x, false), 0)
        .map((x) => x + 1)
        .end();
      expect(result).toBe(1);
    });

    it("works with complex types", () => {
      interface ValidUser extends User {
        verified: boolean;
      }
      function isValidUser(user: User): user is ValidUser {
        return "verified" in user;
      }

      const invalidUser: User = {
        id: 1,
        name: "John",
        age: 30,
      };

      const defaultUser: ValidUser = {
        id: 0,
        name: "Default",
        age: 0,
        verified: false,
      };

      const result = pipe<User>(invalidUser)
        .filterWithDefault(isValidUser, defaultUser)
        .map((u) => u.verified)
        .end();

      expect(result).toBe(false);
    });

    it("maintains type narrowing in subsequent operations", () => {
      const result = pipe<string | number>("hello")
        .filterWithDefault((x) => isNumber(x, false), 5)
        .map((x) => x * 2)
        .end();
      expect(result).toBe(10);
    });
  });

  describe("filterResult()", () => {
    it("returns success result when predicate is true", () => {
      const result = pipe<unknown>(42)
        .filterResult((x) => isNumber(x, false))
        .map((result) => {
          if (result.type === "success") {
            return result.value + 1;
          }
          return 0;
        })
        .end();
      expect(result).toBe(43);
    });

    it("returns error result when predicate is false", () => {
      const result = pipe<unknown>("not a number")
        .filterResult((x) => isNumber(x, false))
        .map((result) => {
          if (result.type === "error") {
            return result.error.message;
          }
          return "";
        })
        .end();
      expect(result).toBe("Value did not match filter predicate");
    });

    it("can be used with complex types", () => {
      interface ValidUser extends User {
        verified: boolean;
      }
      function isValidUser(user: User): user is ValidUser {
        return "verified" in user;
      }

      const validUser: ValidUser = {
        id: 1,
        name: "John",
        age: 30,
        verified: true,
      };

      const result = pipe<User>(validUser)
        .filterResult(isValidUser)
        .map((result) => {
          if (result.type === "success") {
            return result.value.verified;
          }
          return false;
        })
        .end();

      expect(result).toBe(true);
    });

    it("can be chained with other operations", () => {
      const result = pipe<unknown>(5)
        .filterResult((x) => isNumber(x, false))
        .map((result) => {
          if (result.type === "success") {
            return result.value * 2;
          }
          return 0;
        })
        .map((x) => x + 1)
        .end();

      expect(result).toBe(11);
    });

    it("handles error cases gracefully", () => {
      const processValue = (value: unknown) => {
        return pipe<unknown>(value)
          .filterResult((x) => isNumber(x, false))
          .map((result) => {
            if (result.type === "success") {
              return result.value * 2;
            }
            return 0;
          })
          .end();
      };

      expect(processValue(10)).toBe(20);
      expect(processValue("not a number")).toBe(0);
      expect(processValue(null)).toBe(0);
    });
  });
});

import { createPipeline } from "@/Tool/createPipeline";

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

describe("createPipeline", () => {
  it("returns initial value when called without arguments", () => {
    const pipeline = createPipeline(1);
    expect(pipeline()).toBe(1);
  });

  it("returns a new Pipeline instance when passing a function as argument", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x) => x + 1);
    expect(newPipeline).toBeInstanceOf(Function);
  });

  it("returns transformed value when called with a function argument", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x) => x + 1);
    expect(newPipeline()).toBe(2);
  });

  it("can chain multiple function calls", () => {
    const pipeline = createPipeline(1);
    const result = pipeline((x) => x + 1)((x) => x * 2)((x) => x - 1)();
    expect(result).toBe(3);
  });

  it("correctly infers function types", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x: number) => `${x}`);
    const result: string = newPipeline();
    expect(result).toBe("1");
  });

  it("correctly handles complex type inference", () => {
    const user: User = {
      id: 1,
      name: "John Doe",
      age: 30,
    };

    const pipeline = createPipeline(user);

    const post: Post = pipeline((user: User) => ({
      id: 1,
      title: "First Post",
      content: "Hello, world!",
      author: user,
    }))();

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
    const pipeline = createPipeline<User[]>([]);

    const users: User[] = pipeline((users: User[]) => [
      ...users,
      { id: 1, name: "John Doe", age: 30 },
    ])((users: User[]) => [...users, { id: 2, name: "Jane Smith", age: 25 }])();

    expect(users).toEqual([
      { id: 1, name: "John Doe", age: 30 },
      { id: 2, name: "Jane Smith", age: 25 },
    ]);
  });

  it("works correctly with null as initial value", () => {
    const pipeline = createPipeline(null);
    expect(pipeline()).toBeNull();
  });

  it("works correctly with undefined as initial value", () => {
    const pipeline = createPipeline(undefined);
    expect(pipeline()).toBeUndefined();
  });

  it("can process empty strings correctly", () => {
    const pipeline = createPipeline("");
    const result = pipeline((x) => `${x}test`)((x) => x.toUpperCase())();
    expect(result).toBe("TEST");
  });

  it("can process numeric zero correctly", () => {
    const pipeline = createPipeline(0);
    const result = pipeline((x) => x + 1)((x) => x * 2)();
    expect(result).toBe(2);
  });

  it("can process boolean transformations correctly", () => {
    const pipeline = createPipeline(true);
    const result = pipeline((x) => !x)((x) => x.toString())();
    expect(result).toBe("false");
  });

  it("can process array transformations correctly", () => {
    const pipeline = createPipeline([1, 2, 3, 4, 5]);
    const result = pipeline((arr) => arr.filter((x) => x % 2 === 0))((arr) =>
      arr.map((x) => x * 2),
    )();
    expect(result).toEqual([4, 8]);
  });

  it("can process complex object transformations correctly", () => {
    interface Data {
      count: number;
      items: string[];
    }

    const initial: Data = { count: 0, items: [] };
    const pipeline = createPipeline(initial);

    const result = pipeline((data) => ({ ...data, count: data.count + 1 }))(
      (data) => ({
        ...data,
        items: [...data.items, `Item ${data.count}`],
      }),
    )();

    expect(result).toEqual({
      count: 1,
      items: ["Item 1"],
    });
  });

  it("can process multiple type transformations in chain", () => {
    const pipeline = createPipeline(123);
    const result = pipeline((x: number) => x.toString())((x: string) =>
      x.split(""),
    )((arr: string[]) => arr.map(Number))();

    expect(result).toEqual([1, 2, 3]);
  });
});

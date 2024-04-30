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
  it("引数なしで呼び出すと、初期値を返す", () => {
    const pipeline = createPipeline(1);
    expect(pipeline()).toBe(1);
  });

  it("関数を引数として渡すと、新しいPipelineインスタンスを返す", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x) => x + 1);
    expect(newPipeline).toBeInstanceOf(Function);
  });

  it("関数を引数として渡して呼び出すと、変換された値を返す", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x) => x + 1);
    expect(newPipeline()).toBe(2);
  });

  it("複数の関数を連鎖して呼び出すことができる", () => {
    const pipeline = createPipeline(1);
    const result = pipeline((x) => x + 1)((x) => x * 2)((x) => x - 1)();
    expect(result).toBe(3);
  });

  it("関数の型が正しく推論される", () => {
    const pipeline = createPipeline(1);
    const newPipeline = pipeline((x: number) => `${x}`);
    const result: string = newPipeline();
    expect(result).toBe("1");
  });

  it("複雑な型の推論が正しく行われる", () => {
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

  it("ジェネリック型のネストが正しく推論される", () => {
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
});

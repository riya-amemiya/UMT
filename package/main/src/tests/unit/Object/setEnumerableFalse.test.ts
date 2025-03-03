import { setEnumerableFalse } from "@/Object/setEnumerableFalse";

describe("setEnumerableFalse", () => {
  it("プロパティをenumerable:falseに設定すること", () => {
    const obj = { test: "value" };
    setEnumerableFalse(obj, "test");

    const descriptor = Object.getOwnPropertyDescriptor(obj, "test");
    expect(descriptor?.enumerable).toBe(false);
  });

  it("プロパティの値が保持されること", () => {
    const obj = { test: "value" };
    setEnumerableFalse(obj, "test");

    expect(obj.test).toBe("value");
  });

  it("オブジェクトが返されること", () => {
    const obj = { test: "value" };
    const result = setEnumerableFalse(obj, "test");

    expect(result).toBe(obj);
  });

  it("for...inでプロパティが列挙されないこと", () => {
    const obj = { test: "value", other: "other" };
    setEnumerableFalse(obj, "test");

    const keys: string[] = [];
    for (const key in obj) {
      keys.push(key);
    }

    expect(keys).toEqual(["other"]);
  });
});

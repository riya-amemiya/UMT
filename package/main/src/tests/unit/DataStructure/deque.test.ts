import { Deque } from "@/DataStructure/deque";

describe("Deque", () => {
  it("should start empty", () => {
    const deque = new Deque<number>();
    expect(deque.size).toBe(0);
    expect(deque.isEmpty).toBe(true);
    expect(deque.popFront()).toBeUndefined();
    expect(deque.popBack()).toBeUndefined();
    expect(deque.peekFront()).toBeUndefined();
    expect(deque.peekBack()).toBeUndefined();
    expect([...deque]).toEqual([]);
  });

  it("should push and pop at the back like a stack", () => {
    const deque = new Deque<number>();
    deque.pushBack(1);
    deque.pushBack(2);
    deque.pushBack(3);
    expect(deque.peekBack()).toBe(3);
    expect(deque.popBack()).toBe(3);
    expect(deque.popBack()).toBe(2);
    expect(deque.popBack()).toBe(1);
    expect(deque.isEmpty).toBe(true);
  });

  it("should push at the back and pop at the front like a queue", () => {
    const deque = new Deque<string>();
    deque.pushBack("a");
    deque.pushBack("b");
    expect(deque.peekFront()).toBe("a");
    expect(deque.popFront()).toBe("a");
    expect(deque.popFront()).toBe("b");
    expect(deque.popFront()).toBeUndefined();
  });

  it("should push at the front", () => {
    const deque = new Deque<number>();
    deque.pushFront(3);
    deque.pushFront(2);
    deque.pushFront(1);
    expect([...deque]).toEqual([1, 2, 3]);
    expect(deque.peekFront()).toBe(1);
    expect(deque.peekBack()).toBe(3);
    expect(deque.size).toBe(3);
    expect(deque.isEmpty).toBe(false);
  });

  it("should mix operations at both ends", () => {
    const deque = new Deque<number>();
    deque.pushBack(2);
    deque.pushFront(1);
    deque.pushBack(3);
    deque.pushFront(0);
    expect([...deque]).toEqual([0, 1, 2, 3]);
    expect(deque.popBack()).toBe(3);
    expect(deque.popFront()).toBe(0);
    expect([...deque]).toEqual([1, 2]);
  });

  it("should keep order when growing beyond the initial capacity", () => {
    const deque = new Deque<number>();
    for (let index = 0; index < 20; index++) {
      deque.pushBack(index);
    }
    for (let index = 1; index <= 20; index++) {
      deque.pushFront(-index);
    }
    expect(deque.size).toBe(40);
    expect([...deque]).toEqual([
      ...Array.from({ length: 20 }, (_, index) => index - 20),
      ...Array.from({ length: 20 }, (_, index) => index),
    ]);
    expect(deque.peekFront()).toBe(-20);
    expect(deque.peekBack()).toBe(19);
  });

  it("should keep order when growing while wrapped around", () => {
    const deque = new Deque<number>();
    for (let index = 0; index < 10; index++) {
      deque.pushBack(index);
    }
    for (let index = 0; index < 8; index++) {
      deque.popFront();
    }
    for (let index = 10; index < 30; index++) {
      deque.pushBack(index);
    }
    expect([...deque]).toEqual(
      Array.from({ length: 22 }, (_, index) => index + 8),
    );
  });

  it("should store undefined values", () => {
    const deque = new Deque<number | undefined>();
    deque.pushBack(undefined);
    expect(deque.size).toBe(1);
    expect(deque.popFront()).toBeUndefined();
    expect(deque.size).toBe(0);
  });

  it("should clear all elements", () => {
    const deque = new Deque<number>();
    for (let index = 0; index < 30; index++) {
      deque.pushBack(index);
    }
    deque.clear();
    expect(deque.size).toBe(0);
    expect(deque.isEmpty).toBe(true);
    expect([...deque]).toEqual([]);
    deque.pushFront(1);
    expect([...deque]).toEqual([1]);
  });

  it("should support for...of iteration", () => {
    const deque = new Deque<string>();
    deque.pushBack("b");
    deque.pushFront("a");
    const values: string[] = [];
    for (const value of deque) {
      values.push(value);
    }
    expect(values).toEqual(["a", "b"]);
  });
});

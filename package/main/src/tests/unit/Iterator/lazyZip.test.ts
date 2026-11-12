import { lazyZip } from "@/Iterator/lazyZip";

describe("lazyZip", () => {
  it("combines values at corresponding positions", () => {
    expect([...lazyZip([1, 2, 3], ["a", "b", "c"])]).toEqual([
      [1, "a"],
      [2, "b"],
      [3, "c"],
    ]);
  });

  it("stops at the shortest iterable", () => {
    expect([...lazyZip([1, 2, 3], ["a", "b"], [true])]).toEqual([
      [1, "a", true],
    ]);
  });

  it("yields nothing without iterables", () => {
    expect([...lazyZip()]).toEqual([]);
  });

  it("yields single-element tuples for one iterable", () => {
    expect([...lazyZip(new Set([1, 2]))]).toEqual([[1], [2]]);
  });

  it("yields nothing when any iterable is empty", () => {
    expect([...lazyZip([1, 2], [])]).toEqual([]);
  });

  it("works with infinite iterables", () => {
    function* naturals() {
      let n = 0;
      while (true) {
        yield n;
        n += 1;
      }
    }
    expect([...lazyZip(naturals(), ["a", "b"])]).toEqual([
      [0, "a"],
      [1, "b"],
    ]);
  });

  it("closes every source iterator when exhausted", () => {
    const closed: string[] = [];
    function* tracked(name: string, values: number[]) {
      try {
        yield* values;
      } finally {
        closed.push(name);
      }
    }
    expect([
      ...lazyZip(tracked("first", [1, 2, 3]), tracked("second", [4])),
    ]).toEqual([[1, 4]]);
    expect(closed.sort()).toEqual(["first", "second"]);
  });

  it("closes every source iterator when the consumer stops early", () => {
    const closed: string[] = [];
    function* tracked(name: string) {
      try {
        let n = 0;
        while (true) {
          yield n;
          n += 1;
        }
      } finally {
        closed.push(name);
      }
    }
    const iterator = lazyZip(tracked("first"), tracked("second"));
    expect(iterator.next().value).toEqual([0, 0]);
    iterator.return();
    expect(closed.sort()).toEqual(["first", "second"]);
  });
});

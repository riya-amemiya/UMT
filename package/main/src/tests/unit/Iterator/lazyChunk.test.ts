import { lazyChunk } from "@/Iterator/lazyChunk";

describe("lazyChunk", () => {
  it("groups values into chunks of the given size", () => {
    expect([...lazyChunk([1, 2, 3, 4, 5, 6], 2)]).toEqual([
      [1, 2],
      [3, 4],
      [5, 6],
    ]);
  });

  it("yields a shorter last chunk", () => {
    expect([...lazyChunk([1, 2, 3, 4, 5], 2)]).toEqual([[1, 2], [3, 4], [5]]);
  });

  it("yields one chunk when size exceeds the length", () => {
    expect([...lazyChunk([1, 2], 5)]).toEqual([[1, 2]]);
  });

  it("yields nothing for an empty iterable", () => {
    expect([...lazyChunk([], 3)]).toEqual([]);
  });

  it("yields independent arrays", () => {
    const chunks = [...lazyChunk([1, 2, 3, 4], 2)];
    chunks[0].push(99);
    expect(chunks[1]).toEqual([3, 4]);
  });

  it("works with infinite iterables and stops early", () => {
    let pulled = 0;
    function* naturals() {
      let n = 1;
      while (true) {
        pulled += 1;
        yield n;
        n += 1;
      }
    }
    const iterator = lazyChunk(naturals(), 3);
    expect(iterator.next().value).toEqual([1, 2, 3]);
    expect(iterator.next().value).toEqual([4, 5, 6]);
    expect(pulled).toBe(6);
  });

  it("works with a Set", () => {
    expect([...lazyChunk(new Set(["a", "b", "c"]), 2)]).toEqual([
      ["a", "b"],
      ["c"],
    ]);
  });
});

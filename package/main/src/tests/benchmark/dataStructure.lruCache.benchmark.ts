import { bench, run, summary, do_not_optimize } from "mitata";
import { LRUCache } from "@/DataStructure/lruCache";

const CACHE_SIZES = [100, 1000, 10_000];

for (const size of CACHE_SIZES) {
  summary(() => {
    bench(`LRUCache set (capacity=${size})`, () => {
      const cache = new LRUCache<number, number>(size);
      for (let i = 0; i < size; i++) {
        cache.set(i, i);
      }
      do_not_optimize(cache);
    });

    bench(`plain Map set (limit=${size})`, () => {
      const map = new Map<number, number>();
      for (let i = 0; i < size; i++) {
        if (map.size >= size) {
          const firstKey = map.keys().next().value;
          if (firstKey !== undefined) {
            map.delete(firstKey);
          }
        }
        map.set(i, i);
      }
      do_not_optimize(map);
    });
  });

  summary(() => {
    const lruCache = new LRUCache<number, number>(size);
    const plainMap = new Map<number, number>();
    for (let i = 0; i < size; i++) {
      lruCache.set(i, i);
      plainMap.set(i, i);
    }

    bench(`LRUCache get (capacity=${size})`, () => {
      for (let i = 0; i < size; i++) {
        do_not_optimize(lruCache.get(i));
      }
    });

    bench(`plain Map get (limit=${size})`, () => {
      for (let i = 0; i < size; i++) {
        do_not_optimize(plainMap.get(i));
      }
    });
  });

  summary(() => {
    bench(`LRUCache set+get mixed (capacity=${size})`, () => {
      const cache = new LRUCache<number, number>(size);
      for (let i = 0; i < size * 2; i++) {
        cache.set(i, i);
        if (i > 0) {
          do_not_optimize(cache.get(i - 1));
        }
      }
    });

    bench(`plain Map set+get mixed (limit=${size})`, () => {
      const map = new Map<number, number>();
      for (let i = 0; i < size * 2; i++) {
        if (map.size >= size) {
          const firstKey = map.keys().next().value;
          if (firstKey !== undefined) {
            map.delete(firstKey);
          }
        }
        map.set(i, i);
        if (i > 0) {
          do_not_optimize(map.get(i - 1));
        }
      }
    });
  });
}

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();

import { bench, run, summary, do_not_optimize } from "mitata";
import { LRUCache } from "@/DataStructure/lruCache";

const CACHE_SIZES = [100, 1000, 10_000];

// The smaller caches operate in microseconds, which is buried in CI-runner
// jitter. Each operation is repeated a fixed number of times so one measured
// sample reaches a few milliseconds and stays above that jitter. The set/get
// passes scale with the capacity, while the mixed pass already does twice the
// work, so it uses a smaller count to avoid overshooting on the largest cache.
// The counts are fixed so the base/head comparison stays meaningful.
const repsSetGet = (size: number): number => {
  if (size <= 100) {
    return 200;
  }
  if (size <= 1000) {
    return 20;
  }
  return 13;
};
const repsMixed = (size: number): number => {
  if (size <= 100) {
    return 200;
  }
  if (size <= 1000) {
    return 12;
  }
  return 1;
};

for (const size of CACHE_SIZES) {
  const setGetReps = repsSetGet(size);
  const mixedReps = repsMixed(size);

  summary(() => {
    bench(`LRUCache set (capacity=${size})`, () => {
      for (let r = 0; r < setGetReps; r++) {
        const cache = new LRUCache<number, number>(size);
        for (let i = 0; i < size; i++) {
          cache.set(i, i);
        }
        do_not_optimize(cache);
      }
    });

    bench(`plain Map set (limit=${size})`, () => {
      for (let r = 0; r < setGetReps; r++) {
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
      }
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
      for (let r = 0; r < setGetReps; r++) {
        for (let i = 0; i < size; i++) {
          do_not_optimize(lruCache.get(i));
        }
      }
    });

    bench(`plain Map get (limit=${size})`, () => {
      for (let r = 0; r < setGetReps; r++) {
        for (let i = 0; i < size; i++) {
          do_not_optimize(plainMap.get(i));
        }
      }
    });
  });

  summary(() => {
    bench(`LRUCache set+get mixed (capacity=${size})`, () => {
      for (let r = 0; r < mixedReps; r++) {
        const cache = new LRUCache<number, number>(size);
        for (let i = 0; i < size * 2; i++) {
          cache.set(i, i);
          if (i > 0) {
            do_not_optimize(cache.get(i - 1));
          }
        }
      }
    });

    bench(`plain Map set+get mixed (limit=${size})`, () => {
      for (let r = 0; r < mixedReps; r++) {
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

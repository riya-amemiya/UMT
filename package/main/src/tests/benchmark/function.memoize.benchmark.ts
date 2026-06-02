import { bench, run, summary, do_not_optimize } from "mitata";
import { memoize as customMemoize } from "@/Function/memoize";
import { memoize as lodashMemoize } from "lodash";
import { memoize as esToolkitMemoize } from "es-toolkit/compat";

const expensive = (n: number) => {
  let result = 0;
  for (let i = 0; i < 100; i++) {
    result += Math.sqrt(n + i);
  }
  return result;
};

// The cache-hit and cache-miss paths differ by an order of magnitude in cost,
// so each group repeats its work enough to bring one measured sample to
// roughly ~500ms. This keeps fixed measurement overhead negligible, and the
// count is shared across variants within a group to keep the comparison fair.
const CACHE_HIT_ITERATIONS = 56_000;
const CACHE_MISS_ITERATIONS = 22_000;

summary(() => {
  bench("customMemoize (cache hit)", () => {
    for (let r = 0; r < CACHE_HIT_ITERATIONS; r++) {
      const memoized = customMemoize(expensive);
      memoized(42);
      for (let i = 0; i < 1000; i++) {
        do_not_optimize(memoized(42));
      }
    }
  });

  bench("lodashMemoize (cache hit)", () => {
    for (let r = 0; r < CACHE_HIT_ITERATIONS; r++) {
      const memoized = lodashMemoize(expensive);
      memoized(42);
      for (let i = 0; i < 1000; i++) {
        do_not_optimize(memoized(42));
      }
    }
  });

  bench("esToolkitMemoize (cache hit)", () => {
    for (let r = 0; r < CACHE_HIT_ITERATIONS; r++) {
      const memoized = esToolkitMemoize(expensive);
      memoized(42);
      for (let i = 0; i < 1000; i++) {
        do_not_optimize(memoized(42));
      }
    }
  });
});

summary(() => {
  bench("customMemoize (cache miss)", () => {
    for (let r = 0; r < CACHE_MISS_ITERATIONS; r++) {
      const memoized = customMemoize(expensive);
      for (let i = 0; i < 100; i++) {
        do_not_optimize(memoized(i));
      }
    }
  });

  bench("lodashMemoize (cache miss)", () => {
    for (let r = 0; r < CACHE_MISS_ITERATIONS; r++) {
      const memoized = lodashMemoize(expensive);
      for (let i = 0; i < 100; i++) {
        do_not_optimize(memoized(i));
      }
    }
  });

  bench("esToolkitMemoize (cache miss)", () => {
    for (let r = 0; r < CACHE_MISS_ITERATIONS; r++) {
      const memoized = esToolkitMemoize(expensive);
      for (let i = 0; i < 100; i++) {
        do_not_optimize(memoized(i));
      }
    }
  });
});

(async () => {
  try {
    await run();
    console.log("Benchmark finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();

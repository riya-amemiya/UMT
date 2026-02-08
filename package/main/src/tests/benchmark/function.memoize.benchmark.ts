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

summary(() => {
  bench("customMemoize (cache hit)", () => {
    const memoized = customMemoize(expensive);
    memoized(42);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(memoized(42));
    }
  });

  bench("lodashMemoize (cache hit)", () => {
    const memoized = lodashMemoize(expensive);
    memoized(42);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(memoized(42));
    }
  });

  bench("esToolkitMemoize (cache hit)", () => {
    const memoized = esToolkitMemoize(expensive);
    memoized(42);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(memoized(42));
    }
  });

  bench("customMemoize (cache miss)", () => {
    const memoized = customMemoize(expensive);
    for (let i = 0; i < 100; i++) {
      do_not_optimize(memoized(i));
    }
  });

  bench("lodashMemoize (cache miss)", () => {
    const memoized = lodashMemoize(expensive);
    for (let i = 0; i < 100; i++) {
      do_not_optimize(memoized(i));
    }
  });

  bench("esToolkitMemoize (cache miss)", () => {
    const memoized = esToolkitMemoize(expensive);
    for (let i = 0; i < 100; i++) {
      do_not_optimize(memoized(i));
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

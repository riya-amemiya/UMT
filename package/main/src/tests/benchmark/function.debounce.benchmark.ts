import { bench, run, summary, do_not_optimize } from "mitata";
import { debounce as customDebounce } from "@/Function/debounce";
import { debounce as lodashDebounce } from "lodash";
import { debounce as esToolkitDebounce } from "es-toolkit";

const noop = () => {
  /* intentionally empty */
};

// Creation costs nanoseconds while a 100-call invocation burst costs a few
// microseconds, so each group uses its own repetition count to bring one
// measured sample to roughly ~500ms. This keeps fixed measurement overhead
// negligible, and the count is shared across variants within a group.
const CREATION_ITERATIONS = 8_000_000;
const INVOCATION_ITERATIONS = 17_500;

summary(() => {
  bench("customDebounce creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(customDebounce(noop, 100));
    }
  });

  bench("lodashDebounce creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(lodashDebounce(noop, 100));
    }
  });

  bench("esToolkitDebounce creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(esToolkitDebounce(noop, 100));
    }
  });
});

summary(() => {
  bench("customDebounce invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const debounced = customDebounce(noop, 100);
      for (let j = 0; j < 100; j++) {
        debounced();
      }
      debounced.cancel();
    }
  });

  bench("lodashDebounce invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const debounced = lodashDebounce(noop, 100);
      for (let j = 0; j < 100; j++) {
        debounced();
      }
      debounced.cancel();
    }
  });

  bench("esToolkitDebounce invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const debounced = esToolkitDebounce(noop, 100);
      for (let j = 0; j < 100; j++) {
        debounced();
      }
      debounced.cancel();
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

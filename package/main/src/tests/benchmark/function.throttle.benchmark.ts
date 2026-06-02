import { bench, run, summary, do_not_optimize } from "mitata";
import { throttle as customThrottle } from "@/Function/throttle";
import { throttle as lodashThrottle } from "lodash";
import { throttle as esToolkitThrottle } from "es-toolkit";

const noop = () => {
  /* intentionally empty */
};

// Creation costs nanoseconds while a 100-call invocation burst costs a few
// microseconds, so each group uses its own repetition count to bring one
// measured sample to roughly ~500ms. This keeps fixed measurement overhead
// negligible, and the count is shared across variants within a group.
const CREATION_ITERATIONS = 5_000_000;
const INVOCATION_ITERATIONS = 14_500;

summary(() => {
  bench("customThrottle creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(customThrottle(noop, 100));
    }
  });

  bench("lodashThrottle creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(lodashThrottle(noop, 100));
    }
  });

  bench("esToolkitThrottle creation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      do_not_optimize(esToolkitThrottle(noop, 100));
    }
  });
});

summary(() => {
  bench("customThrottle invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const throttled = customThrottle(noop, 100);
      for (let j = 0; j < 100; j++) {
        throttled();
      }
      throttled.cancel();
    }
  });

  bench("lodashThrottle invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const throttled = lodashThrottle(noop, 100);
      for (let j = 0; j < 100; j++) {
        throttled();
      }
      throttled.cancel();
    }
  });

  bench("esToolkitThrottle invocation", () => {
    for (let i = 0; i < INVOCATION_ITERATIONS; i++) {
      const throttled = esToolkitThrottle(noop, 100);
      for (let j = 0; j < 100; j++) {
        throttled();
      }
      throttled.cancel();
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

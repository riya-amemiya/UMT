import { bench, run, summary, do_not_optimize } from "mitata";
import { once as customOnce } from "@/Function/once";
import { once as lodashOnce } from "lodash";
import { once as esToolkitOnce } from "es-toolkit";

const factory = () => 42;

// Creating and invoking a wrapper costs only nanoseconds, so the raw figure is
// noise-dominated. Each group repeats its operation enough times for one
// measured sample to take roughly ~500ms, keeping the timer resolution and
// scheduling jitter negligible. Counts differ per group because the per-call
// cost differs by two orders of magnitude, and are shared across variants
// within a group so the comparison stays fair.
const CREATION_ITERATIONS = 27_000_000;
const REPEATED_ITERATIONS = 260_000;

summary(() => {
  bench("customOnce creation + invocation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      const fn = customOnce(factory);
      do_not_optimize(fn());
    }
  });

  bench("lodashOnce creation + invocation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      const fn = lodashOnce(factory);
      do_not_optimize(fn());
    }
  });

  bench("esToolkitOnce creation + invocation", () => {
    for (let i = 0; i < CREATION_ITERATIONS; i++) {
      const fn = esToolkitOnce(factory);
      do_not_optimize(fn());
    }
  });
});

summary(() => {
  bench("customOnce repeated invocation", () => {
    for (let i = 0; i < REPEATED_ITERATIONS; i++) {
      const fn = customOnce(factory);
      for (let j = 0; j < 1000; j++) {
        do_not_optimize(fn());
      }
    }
  });

  bench("lodashOnce repeated invocation", () => {
    for (let i = 0; i < REPEATED_ITERATIONS; i++) {
      const fn = lodashOnce(factory);
      for (let j = 0; j < 1000; j++) {
        do_not_optimize(fn());
      }
    }
  });

  bench("esToolkitOnce repeated invocation", () => {
    for (let i = 0; i < REPEATED_ITERATIONS; i++) {
      const fn = esToolkitOnce(factory);
      for (let j = 0; j < 1000; j++) {
        do_not_optimize(fn());
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

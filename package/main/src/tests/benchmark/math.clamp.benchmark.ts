import { bench, run, summary, do_not_optimize } from "mitata";
import { clamp as customClamp } from "@/Math/clamp";
import { clamp as lodashClamp } from "lodash";
import { clamp as esToolkitClamp } from "es-toolkit";

const testValues = [-100, -10, 0, 5, 50, 100, 1000];

// A single clamp call costs only a few nanoseconds, which is far below the
// timer resolution and dominated by scheduling jitter. Repeating the operation
// this many times makes one measured sample take roughly ~500ms so that fixed
// measurement overhead becomes negligible. The same count is shared across all
// variants to keep the relative comparison fair.
const ITERATIONS = 34_000_000;

summary(() => {
  bench("customClamp", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(customClamp(value, 0, 10));
      }
    }
  });

  bench("lodashClamp", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(lodashClamp(value, 0, 10));
      }
    }
  });

  bench("esToolkitClamp", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(esToolkitClamp(value, 0, 10));
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

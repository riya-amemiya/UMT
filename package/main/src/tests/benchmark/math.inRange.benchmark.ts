import { bench, run, summary, do_not_optimize } from "mitata";
import { inRange as customInRange } from "@/Math/inRange";
import { inRange as lodashInRange } from "lodash";
import { inRange as esToolkitInRange } from "es-toolkit";

const testValues = [-10, 0, 3, 5, 10, 50];

// A single inRange call runs in a handful of nanoseconds, which is buried in
// timer resolution and CPU jitter. Repeating it this many times makes one
// measured sample take roughly ~500ms so that fixed overhead is negligible.
// The same count is shared across all variants to keep the comparison fair.
const ITERATIONS = 40_000_000;

summary(() => {
  bench("customInRange", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(customInRange(value, 0, 10));
      }
    }
  });

  bench("lodashInRange", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(lodashInRange(value, 0, 10));
      }
    }
  });

  bench("esToolkitInRange", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      for (const value of testValues) {
        do_not_optimize(esToolkitInRange(value, 0, 10));
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

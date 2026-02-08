import { bench, run, summary, do_not_optimize } from "mitata";
import { inRange as customInRange } from "@/Math/inRange";
import { inRange as lodashInRange } from "lodash";
import { inRange as esToolkitInRange } from "es-toolkit";

const testValues = [-10, 0, 3, 5, 10, 50];

summary(() => {
  bench("customInRange", () => {
    for (const value of testValues) {
      do_not_optimize(customInRange(value, 0, 10));
    }
  });

  bench("lodashInRange", () => {
    for (const value of testValues) {
      do_not_optimize(lodashInRange(value, 0, 10));
    }
  });

  bench("esToolkitInRange", () => {
    for (const value of testValues) {
      do_not_optimize(esToolkitInRange(value, 0, 10));
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

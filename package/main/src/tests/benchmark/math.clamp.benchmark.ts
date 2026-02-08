import { bench, run, summary, do_not_optimize } from "mitata";
import { clamp as customClamp } from "@/Math/clamp";
import { clamp as lodashClamp } from "lodash";
import { clamp as esToolkitClamp } from "es-toolkit";

const testValues = [-100, -10, 0, 5, 50, 100, 1000];

summary(() => {
  bench("customClamp", () => {
    for (const value of testValues) {
      do_not_optimize(customClamp(value, 0, 10));
    }
  });

  bench("lodashClamp", () => {
    for (const value of testValues) {
      do_not_optimize(lodashClamp(value, 0, 10));
    }
  });

  bench("esToolkitClamp", () => {
    for (const value of testValues) {
      do_not_optimize(esToolkitClamp(value, 0, 10));
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

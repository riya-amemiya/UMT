import { bench, run, summary, do_not_optimize } from "mitata";
import { deepClone as customDeepClone } from "@/Object/deepClone";
import { cloneDeep as lodashCloneDeep } from "lodash";
import { cloneDeep as esToolkitCloneDeep } from "es-toolkit";

const shallowObj = { a: 1, b: "hello", c: true, d: null, e: 42 };

const deepObj = {
  a: { b: { c: { d: { e: 1 } } } },
  f: [1, 2, [3, [4]]],
  g: "string",
  h: new Date(),
};

// Cloning these small objects costs from under a microsecond (shallow) to a few
// microseconds (deep), which is noise-dominated. Each group repeats the clone
// enough times to bring one measured sample to roughly ~500ms so that fixed
// overhead is negligible. The count is shared across variants within a group.
const SHALLOW_ITERATIONS = 1_100_000;
const DEEP_ITERATIONS = 120_000;

summary(() => {
  bench("customDeepClone (shallow)", () => {
    for (let i = 0; i < SHALLOW_ITERATIONS; i++) {
      do_not_optimize(customDeepClone(shallowObj));
    }
  });

  bench("lodashCloneDeep (shallow)", () => {
    for (let i = 0; i < SHALLOW_ITERATIONS; i++) {
      do_not_optimize(lodashCloneDeep(shallowObj));
    }
  });

  bench("esToolkitCloneDeep (shallow)", () => {
    for (let i = 0; i < SHALLOW_ITERATIONS; i++) {
      do_not_optimize(esToolkitCloneDeep(shallowObj));
    }
  });
});

summary(() => {
  bench("customDeepClone (deep)", () => {
    for (let i = 0; i < DEEP_ITERATIONS; i++) {
      do_not_optimize(customDeepClone(deepObj));
    }
  });

  bench("lodashCloneDeep (deep)", () => {
    for (let i = 0; i < DEEP_ITERATIONS; i++) {
      do_not_optimize(lodashCloneDeep(deepObj));
    }
  });

  bench("esToolkitCloneDeep (deep)", () => {
    for (let i = 0; i < DEEP_ITERATIONS; i++) {
      do_not_optimize(esToolkitCloneDeep(deepObj));
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

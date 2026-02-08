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

summary(() => {
  bench("customDeepClone (shallow)", () => {
    do_not_optimize(customDeepClone(shallowObj));
  });

  bench("lodashCloneDeep (shallow)", () => {
    do_not_optimize(lodashCloneDeep(shallowObj));
  });

  bench("esToolkitCloneDeep (shallow)", () => {
    do_not_optimize(esToolkitCloneDeep(shallowObj));
  });

  bench("customDeepClone (deep)", () => {
    do_not_optimize(customDeepClone(deepObj));
  });

  bench("lodashCloneDeep (deep)", () => {
    do_not_optimize(lodashCloneDeep(deepObj));
  });

  bench("esToolkitCloneDeep (deep)", () => {
    do_not_optimize(esToolkitCloneDeep(deepObj));
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

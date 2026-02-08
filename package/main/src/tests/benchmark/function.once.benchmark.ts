import { bench, run, summary, do_not_optimize } from "mitata";
import { once as customOnce } from "@/Function/once";
import { once as lodashOnce } from "lodash";
import { once as esToolkitOnce } from "es-toolkit";

const factory = () => 42;

summary(() => {
  bench("customOnce creation + invocation", () => {
    const fn = customOnce(factory);
    do_not_optimize(fn());
  });

  bench("lodashOnce creation + invocation", () => {
    const fn = lodashOnce(factory);
    do_not_optimize(fn());
  });

  bench("esToolkitOnce creation + invocation", () => {
    const fn = esToolkitOnce(factory);
    do_not_optimize(fn());
  });

  bench("customOnce repeated invocation", () => {
    const fn = customOnce(factory);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(fn());
    }
  });

  bench("lodashOnce repeated invocation", () => {
    const fn = lodashOnce(factory);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(fn());
    }
  });

  bench("esToolkitOnce repeated invocation", () => {
    const fn = esToolkitOnce(factory);
    for (let i = 0; i < 1000; i++) {
      do_not_optimize(fn());
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

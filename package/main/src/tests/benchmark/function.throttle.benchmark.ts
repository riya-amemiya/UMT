import { bench, run, summary, do_not_optimize } from "mitata";
import { throttle as customThrottle } from "@/Function/throttle";
import { throttle as lodashThrottle } from "lodash";
import { throttle as esToolkitThrottle } from "es-toolkit";

const noop = () => {
  /* intentionally empty */
};

summary(() => {
  bench("customThrottle creation", () => {
    do_not_optimize(customThrottle(noop, 100));
  });

  bench("lodashThrottle creation", () => {
    do_not_optimize(lodashThrottle(noop, 100));
  });

  bench("esToolkitThrottle creation", () => {
    do_not_optimize(esToolkitThrottle(noop, 100));
  });

  bench("customThrottle invocation", () => {
    const throttled = customThrottle(noop, 100);
    for (let i = 0; i < 100; i++) {
      throttled();
    }
    throttled.cancel();
  });

  bench("lodashThrottle invocation", () => {
    const throttled = lodashThrottle(noop, 100);
    for (let i = 0; i < 100; i++) {
      throttled();
    }
    throttled.cancel();
  });

  bench("esToolkitThrottle invocation", () => {
    const throttled = esToolkitThrottle(noop, 100);
    for (let i = 0; i < 100; i++) {
      throttled();
    }
    throttled.cancel();
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

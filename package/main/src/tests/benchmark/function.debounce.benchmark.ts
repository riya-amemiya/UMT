import { bench, run, summary, do_not_optimize } from "mitata";
import { debounce as customDebounce } from "@/Function/debounce";
import { debounce as lodashDebounce } from "lodash";
import { debounce as esToolkitDebounce } from "es-toolkit";

const noop = () => {
  /* intentionally empty */
};

summary(() => {
  bench("customDebounce creation", () => {
    do_not_optimize(customDebounce(noop, 100));
  });

  bench("lodashDebounce creation", () => {
    do_not_optimize(lodashDebounce(noop, 100));
  });

  bench("esToolkitDebounce creation", () => {
    do_not_optimize(esToolkitDebounce(noop, 100));
  });

  bench("customDebounce invocation", () => {
    const debounced = customDebounce(noop, 100);
    for (let i = 0; i < 100; i++) {
      debounced();
    }
    debounced.cancel();
  });

  bench("lodashDebounce invocation", () => {
    const debounced = lodashDebounce(noop, 100);
    for (let i = 0; i < 100; i++) {
      debounced();
    }
    debounced.cancel();
  });

  bench("esToolkitDebounce invocation", () => {
    const debounced = esToolkitDebounce(noop, 100);
    for (let i = 0; i < 100; i++) {
      debounced();
    }
    debounced.cancel();
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

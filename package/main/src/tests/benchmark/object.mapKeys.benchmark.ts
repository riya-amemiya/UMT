import { bench, run, summary, do_not_optimize } from "mitata";
import { mapKeys as customMapKeys } from "@/Object/mapKeys";
import { mapKeys as lodashMapKeys } from "lodash";
import { mapKeys as esToolkitMapKeys } from "es-toolkit/compat";

const smallObj: Record<string, number> = {};
for (let i = 0; i < 10; i++) {
  smallObj[`key${i}`] = i;
}

const mediumObj: Record<string, number> = {};
for (let i = 0; i < 100; i++) {
  mediumObj[`key${i}`] = i;
}

const largeObj: Record<string, number> = {};
for (let i = 0; i < 1000; i++) {
  largeObj[`key${i}`] = i;
}

const transform = (_v: unknown, k: string) => k.toUpperCase();

// Each key-count tier is benchmarked in its own group so the three libraries
// are compared on the same input. The repetition count per tier is chosen so
// that one measured sample takes roughly ~500ms, keeping timer resolution and
// scheduling jitter negligible. The count is shared across variants in a tier.
const SMALL_ITERATIONS = 430_000;
const MEDIUM_ITERATIONS = 28_000;
const LARGE_ITERATIONS = 2250;

summary(() => {
  bench("customMapKeys (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(customMapKeys(smallObj, transform));
    }
  });

  bench("lodashMapKeys (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(lodashMapKeys(smallObj, transform));
    }
  });

  bench("esToolkitMapKeys (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapKeys(smallObj, transform));
    }
  });
});

summary(() => {
  bench("customMapKeys (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(customMapKeys(mediumObj, transform));
    }
  });

  bench("lodashMapKeys (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(lodashMapKeys(mediumObj, transform));
    }
  });

  bench("esToolkitMapKeys (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapKeys(mediumObj, transform));
    }
  });
});

summary(() => {
  bench("customMapKeys (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(customMapKeys(largeObj, transform));
    }
  });

  bench("lodashMapKeys (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(lodashMapKeys(largeObj, transform));
    }
  });

  bench("esToolkitMapKeys (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapKeys(largeObj, transform));
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

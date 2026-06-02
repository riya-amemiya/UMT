import { bench, run, summary, do_not_optimize } from "mitata";
import { mapValues as customMapValues } from "@/Object/mapValues";
import { mapValues as lodashMapValues } from "lodash";
import { mapValues as esToolkitMapValues } from "es-toolkit/compat";

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

const transform = (v: unknown) => (v as number) * 2;

// Each key-count tier is benchmarked in its own group so the three libraries
// are compared on the same input. The repetition count per tier is chosen so
// that one measured sample takes roughly ~500ms, keeping timer resolution and
// scheduling jitter negligible. The count is shared across variants in a tier.
const SMALL_ITERATIONS = 2_500_000;
const MEDIUM_ITERATIONS = 61_000;
const LARGE_ITERATIONS = 3800;

summary(() => {
  bench("customMapValues (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(customMapValues(smallObj, transform));
    }
  });

  bench("lodashMapValues (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(lodashMapValues(smallObj, transform));
    }
  });

  bench("esToolkitMapValues (10 keys)", () => {
    for (let i = 0; i < SMALL_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapValues(smallObj, transform));
    }
  });
});

summary(() => {
  bench("customMapValues (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(customMapValues(mediumObj, transform));
    }
  });

  bench("lodashMapValues (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(lodashMapValues(mediumObj, transform));
    }
  });

  bench("esToolkitMapValues (100 keys)", () => {
    for (let i = 0; i < MEDIUM_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapValues(mediumObj, transform));
    }
  });
});

summary(() => {
  bench("customMapValues (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(customMapValues(largeObj, transform));
    }
  });

  bench("lodashMapValues (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(lodashMapValues(largeObj, transform));
    }
  });

  bench("esToolkitMapValues (1000 keys)", () => {
    for (let i = 0; i < LARGE_ITERATIONS; i++) {
      do_not_optimize(esToolkitMapValues(largeObj, transform));
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

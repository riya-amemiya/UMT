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

summary(() => {
  bench("customMapKeys (10 keys)", () => {
    do_not_optimize(customMapKeys(smallObj, transform));
  });

  bench("lodashMapKeys (10 keys)", () => {
    do_not_optimize(lodashMapKeys(smallObj, transform));
  });

  bench("esToolkitMapKeys (10 keys)", () => {
    do_not_optimize(esToolkitMapKeys(smallObj, transform));
  });

  bench("customMapKeys (100 keys)", () => {
    do_not_optimize(customMapKeys(mediumObj, transform));
  });

  bench("lodashMapKeys (100 keys)", () => {
    do_not_optimize(lodashMapKeys(mediumObj, transform));
  });

  bench("esToolkitMapKeys (100 keys)", () => {
    do_not_optimize(esToolkitMapKeys(mediumObj, transform));
  });

  bench("customMapKeys (1000 keys)", () => {
    do_not_optimize(customMapKeys(largeObj, transform));
  });

  bench("lodashMapKeys (1000 keys)", () => {
    do_not_optimize(lodashMapKeys(largeObj, transform));
  });

  bench("esToolkitMapKeys (1000 keys)", () => {
    do_not_optimize(esToolkitMapKeys(largeObj, transform));
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

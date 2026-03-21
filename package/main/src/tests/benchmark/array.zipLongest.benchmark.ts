import { run, bench, summary, do_not_optimize, type k_state } from "mitata";

// Pre-allocated version (optimized)
const zipLongestOptimized = (...arrays: unknown[][]) => {
  const arraysLength = arrays.length;
  if (arraysLength === 0) {
    return [];
  }
  let maxLength = arrays[0].length;
  for (let index = 1; index < arraysLength; index += 1) {
    if (arrays[index].length > maxLength) {
      maxLength = arrays[index].length;
    }
  }
  const result: unknown[][] = Array.from({ length: maxLength });
  for (let index = 0; index < maxLength; index += 1) {
    const tuple: unknown[] = Array.from({ length: arraysLength });
    for (let arrayIndex = 0; arrayIndex < arraysLength; arrayIndex += 1) {
      tuple[arrayIndex] = arrays[arrayIndex][index];
    }
    result[index] = tuple;
  }
  return result;
};

// Push-based version (original)
const zipLongestPush = (...arrays: unknown[][]) => {
  const arraysLength = arrays.length;
  if (arraysLength === 0) {
    return [];
  }
  let maxLength = arrays[0].length;
  for (let index = 1; index < arraysLength; index += 1) {
    if (arrays[index].length > maxLength) {
      maxLength = arrays[index].length;
    }
  }
  const result: unknown[][] = [];
  for (let index = 0; index < maxLength; index += 1) {
    const tuple: unknown[] = [];
    for (let arrayIndex = 0; arrayIndex < arraysLength; arrayIndex += 1) {
      tuple.push(arrays[arrayIndex][index]);
    }
    result.push(tuple);
  }
  return result;
};

const arrayCounts = [3, 10, 50];
const arrayLengths = [100, 1000, 10_000];

const testData = new Map<string, unknown[][]>();
for (const count of arrayCounts) {
  for (const length of arrayLengths) {
    const arrays: unknown[][] = [];
    for (let i = 0; i < count; i++) {
      arrays.push(
        Array.from({ length: length - (i % 3) * 10 }, () => Math.random()),
      );
    }
    testData.set(`${count}-${length}`, arrays);
  }
}

summary(() => {
  bench(
    "zipLongest-prealloc(arrays: $count, length: $length)",
    function* (state: k_state) {
      const count = state.get("count") as number;
      const length = state.get("length") as number;
      const arrays = testData.get(`${count}-${length}`) as unknown[][];
      yield () => {
        do_not_optimize(zipLongestOptimized(...arrays));
      };
    },
  )
    .args({ count: arrayCounts, length: arrayLengths })
    .gc("inner");

  bench(
    "zipLongest-push(arrays: $count, length: $length)",
    function* (state: k_state) {
      const count = state.get("count") as number;
      const length = state.get("length") as number;
      const arrays = testData.get(`${count}-${length}`) as unknown[][];
      yield () => {
        do_not_optimize(zipLongestPush(...arrays));
      };
    },
  )
    .args({ count: arrayCounts, length: arrayLengths })
    .gc("inner");
});

(async () => {
  try {
    await run();
    console.log("Benchmark finished successfully.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();

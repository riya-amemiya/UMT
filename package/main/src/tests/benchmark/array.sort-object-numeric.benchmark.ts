import {
  run,
  bench,
  summary,
  lineplot,
  do_not_optimize,
  type k_state,
} from "mitata";
import { quickSort } from "@/Array/quickSort";
import { dualPivotQuickSort } from "@/Array/dualPivotQuickSort";
import { mergeSort } from "@/Array/mergeSort";
import { timSort } from "@/Array/timSort";

interface Person {
  id: number;
  name: string;
  age: number;
  score: number;
}

const compareById = (a: Person, b: Person): number => a.id - b.id;

const arraySizes = [10, 100, 1000, 10_000, 50_000];

// Generate random data
const generatePerson = (id: number): Person => ({
  id,
  name: `Person${Math.floor(Math.random() * 1000)}`,
  age: Math.floor(Math.random() * 80) + 20,
  score: Math.random() * 100,
});

const personArrays = new Map<number, Person[]>();

for (const size of arraySizes) {
  personArrays.set(
    size,
    Array.from({ length: size }, (_, i) => generatePerson(i)),
  );
}

summary(() => {
  lineplot(() => {
    bench("quickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = personArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: Person[]) {
          do_not_optimize(quickSort(arr, compareById));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("dualPivotQuickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = personArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: Person[]) {
          do_not_optimize(dualPivotQuickSort(arr, compareById));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("timSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = personArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: Person[]) {
          do_not_optimize(timSort(arr, compareById));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("mergeSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = personArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: Person[]) {
          do_not_optimize(mergeSort(arr, compareById));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("Array.sort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = personArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        0() {
          return [...original_array];
        },
        bench(arr: Person[]) {
          do_not_optimize(arr.sort(compareById));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");
  });
});

(async () => {
  try {
    await run();
    console.log("Object numeric field sort benchmark finished.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();

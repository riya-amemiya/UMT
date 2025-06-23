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

interface Product {
  id: string;
  name: string;
  price: number;
  category: string;
  stock: number;
  rating: number;
}

const compareProductComplex = (a: Product, b: Product): number => {
  const catComp = a.category.localeCompare(b.category);
  if (catComp !== 0) return catComp;
  const ratingComp = b.rating - a.rating; // Descending
  if (ratingComp !== 0) return ratingComp;
  return a.price - b.price;
};

const arraySizes = [10, 100, 1000, 10000, 50000];

const generateProduct = (): Product => ({
  id: Math.random().toString(36).substring(2, 9),
  name: `Product${Math.floor(Math.random() * 1000)}`,
  price: Math.floor(Math.random() * 1000) + 1,
  category: ["Electronics", "Clothing", "Food", "Books", "Toys"][
    Math.floor(Math.random() * 5)
  ],
  stock: Math.floor(Math.random() * 100),
  rating: Math.floor(Math.random() * 50) / 10,
});

const productArrays = new Map<number, Product[]>();

for (const size of arraySizes) {
  productArrays.set(
    size,
    Array.from({ length: size }, () => generateProduct()),
  );
}

summary(() => {
  lineplot(() => {
    bench("quickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = productArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: Product[]) {
          do_not_optimize(quickSort(arr, compareProductComplex));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("dualPivotQuickSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = productArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: Product[]) {
          do_not_optimize(dualPivotQuickSort(arr, compareProductComplex));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("timSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = productArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: Product[]) {
          do_not_optimize(timSort(arr, compareProductComplex));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("mergeSort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = productArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: Product[]) {
          do_not_optimize(mergeSort(arr, compareProductComplex));
        },
      };
    })
      .args("size", arraySizes)
      .gc("inner");

    bench("Array.sort($size)", function* (state: k_state) {
      const size = state.get("size") as number;
      const original_array = productArrays.get(size);

      if (!original_array) {
        throw new Error(`No shared array found for size: ${size}`);
      }

      yield {
        [0]() {
          return [...original_array];
        },
        bench(arr: Product[]) {
          do_not_optimize(arr.sort(compareProductComplex));
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
    console.log("Object complex comparison sort benchmark finished.");
  } catch (e) {
    console.error("Error during benchmark execution:", e);
  }
})();

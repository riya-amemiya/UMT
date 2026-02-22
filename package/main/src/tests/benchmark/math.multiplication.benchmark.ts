import { bench, run, summary, do_not_optimize } from "mitata";
import { multiplication } from "@/Math/multiplication";

// Generate test data
const integers = Array.from({ length: 100 }, (_, i) => i + 1);
const decimals = Array.from({ length: 100 }, (_, i) => (i + 1) / 100);
const mixed = [...integers, ...decimals];

// Large arrays
const largeIntegers = Array.from({ length: 1000 }, (_, i) => i + 1);
const largeDecimals = Array.from({ length: 1000 }, (_, i) => (i + 1) / 100);

summary(() => {
  bench("multiplication (100 integers)", () => {
    do_not_optimize(multiplication(...integers));
  });

  bench("multiplication (100 decimals)", () => {
    do_not_optimize(multiplication(...decimals));
  });

  bench("multiplication (200 mixed)", () => {
    do_not_optimize(multiplication(...mixed));
  });

  bench("multiplication (1000 integers)", () => {
    do_not_optimize(multiplication(...largeIntegers));
  });

  bench("multiplication (1000 decimals)", () => {
    do_not_optimize(multiplication(...largeDecimals));
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

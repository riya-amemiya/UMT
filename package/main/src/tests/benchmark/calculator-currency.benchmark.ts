import { bench, run, summary, do_not_optimize } from "mitata";
import { calculatorCore } from "@/Math/calculator/core";

// Use many currency symbols
const currencies = [
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
];
const rates: Record<string, number> = {};
for (const c of currencies) {
  rates[c] = 1;
}

// Create expression with all currencies repeated
// A1 + B1 + ... + Z1 + A2 + B2 ...
const parts: string[] = [];
for (let i = 1; i <= 5; i++) {
  for (const c of currencies) {
    parts.push(`${c}${i}`);
  }
}
const expression = parts.join(" + ");

// A single evaluation takes around twenty microseconds, so repeating it this
// many times makes one measured sample take roughly ~500ms, keeping fixed
// measurement overhead negligible.
const ITERATIONS = 27_000;

summary(() => {
  bench("calculatorCore multi-currency", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(calculatorCore(expression, rates));
    }
  });
});

(async () => {
  try {
    await run();
  } catch (e) {
    console.error(e);
  }
})();

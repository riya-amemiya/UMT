import { bench, run, summary, do_not_optimize } from "mitata";
import { formatString } from "@/String/formatString";

// Test cases exercising parseArguments with varying argument counts
const simpleTemplate = "Price: {price:currency(ja-JP,JPY)}";
const multiArgTemplate = "Value: {num:number(en-US,2,2)}";
const padTemplate = "ID: {id:pad(4,0)}";
const pluralTemplate = "{count:plural(item,items)}";
const quotedTemplate = "Value: {text:wrap('start','end')}";

const simpleData = { price: 1234 };
const multiArgData = { num: 123.456 };
const padData = { id: 42 };
const pluralData = { count: 5 };
const quotedData = { text: "hello" };

const customFormatters = {
  formatters: {
    wrap: (value: unknown, start: string, end: string) =>
      `${start}${String(value)}${end}`,
  },
};

// A single format call ranges from one to forty microseconds depending on the
// spec, which is noise-dominated. A single repetition count is shared across
// all cases so their relative cost stays comparable; it is sized so the most
// expensive case takes roughly ~500ms, keeping fixed overhead negligible.
const ITERATIONS = 14_000;

summary(() => {
  bench("formatString - currency(locale,code)", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(formatString(simpleTemplate, simpleData));
    }
  });

  bench("formatString - number(locale,min,max)", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(formatString(multiArgTemplate, multiArgData));
    }
  });

  bench("formatString - pad(width,char)", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(formatString(padTemplate, padData));
    }
  });

  bench("formatString - plural(singular,plural)", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(formatString(pluralTemplate, pluralData));
    }
  });

  bench("formatString - custom wrap('quoted','args')", () => {
    for (let i = 0; i < ITERATIONS; i++) {
      do_not_optimize(
        formatString(quotedTemplate, quotedData, customFormatters),
      );
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

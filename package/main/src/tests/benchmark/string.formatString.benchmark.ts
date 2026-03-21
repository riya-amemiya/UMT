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

summary(() => {
  bench("formatString - currency(locale,code)", () => {
    do_not_optimize(formatString(simpleTemplate, simpleData));
  });

  bench("formatString - number(locale,min,max)", () => {
    do_not_optimize(formatString(multiArgTemplate, multiArgData));
  });

  bench("formatString - pad(width,char)", () => {
    do_not_optimize(formatString(padTemplate, padData));
  });

  bench("formatString - plural(singular,plural)", () => {
    do_not_optimize(formatString(pluralTemplate, pluralData));
  });

  bench("formatString - custom wrap('quoted','args')", () => {
    do_not_optimize(formatString(quotedTemplate, quotedData, customFormatters));
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

import { applyFormatter } from "./formatString/applyFormatter";
import { defaultFormatters } from "./formatString/defaultFormatters";
import { detectMode } from "./formatString/detectMode";
import { getValue } from "./formatString/getValue";

import type { FormatData } from "$/string/formatData";
import type { FormatOptions } from "$/string/formatOptions";
import type { FormatValue } from "$/string/formatValue";

export function formatString(
  template: string,
  data?: FormatData,
  options?: FormatOptions,
): string;
export function formatString(
  template: string,
  ...values: FormatValue[]
): string;
export function formatString(
  template: string,
  dataOrFirstValue?: FormatData | FormatValue,
  optionsOrSecondValue?: FormatOptions | FormatValue,
  ...restValues: FormatValue[]
): string {
  const escaped = template
    .replaceAll("{{", "\u0000")
    .replaceAll("}}", "\u0001");

  const { data, options } = detectMode(
    restValues,
    dataOrFirstValue,
    optionsOrSecondValue,
  );

  const formatters = { ...defaultFormatters, ...options.formatters };

  const result = escaped.replaceAll(/{([^}]+)}/g, (match, content: string) => {
    const [pathAndFormatter, defaultValue] = content
      .split("|")
      .map((s) => s.trim());
    const [path, ...formatterParts] = pathAndFormatter.split(":");
    const formatterString = formatterParts.join(":");

    let value: unknown;
    if (Array.isArray(data)) {
      const index = Number(path);
      value = Number.isNaN(index) ? undefined : data[index];
    } else {
      value = getValue(data, path);
    }

    if (value === undefined || value === null) {
      if (defaultValue === undefined) {
        return match;
      }
      value = defaultValue;
    }

    if (formatterString) {
      return applyFormatter(value, formatterString, formatters);
    }

    return String(value);
  });

  return result.replaceAll("\u0000", "{").replaceAll("\u0001", "}");
}

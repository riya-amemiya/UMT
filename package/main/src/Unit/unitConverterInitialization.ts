import { division } from "@/Math/division";
import { multiplication } from "@/Math/multiplication";

export const unitConverterInitialization =
  <
    T extends {
      [k in K]: number;
    },
    K extends string | number | symbol,
  >(
    toBaseUnitRatios: T,
  ) =>
  (value: number, from: keyof T, to: keyof T) =>
    multiplication(
      division(value, toBaseUnitRatios[from]),
      toBaseUnitRatios[to],
    );

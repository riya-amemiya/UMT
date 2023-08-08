import { toKelvin } from "@/Math/toKelvin";
test("{toKelvin}", () => {
  expect(toKelvin(0)).toBe(273.15);
});

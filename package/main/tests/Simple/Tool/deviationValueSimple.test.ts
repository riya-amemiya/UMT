import { deviationValueSimple } from "../../../module/Simple/Tool/deviationValueSimple";
test("deviationValueSimple", () => {
  expect(deviationValueSimple(100, 50, 10)).toBe(100);
  expect(deviationValueSimple(80, [60, 70, 80, 95, 95])).toBe(50);
});

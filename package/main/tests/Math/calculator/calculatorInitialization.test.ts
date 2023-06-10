import { calculatorInitialization } from "../../../module/Math/calculator/calculatorInitialization";
test("calculatorInitialization", () => {
  const calculator = calculatorInitialization({ $: 100 });
  expect(calculator("$1")).toBe("100");
  expect(calculator("100 + $1")).toBe("200");
});

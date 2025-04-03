import { calculatorInitialization } from "@/Math/calculator/calculatorInitialization";

describe("calculatorInitialization function", () => {
  it("should initialize a calculator with exchange rates", () => {
    const calculator = calculatorInitialization({ $: 100 });
    expect(calculator("$1")).toBe("100");
    expect(calculator("100 + $1")).toBe("200");
  });

  it("should handle multiple exchange rates", () => {
    const calculator = calculatorInitialization({ $: 100, EUR: 120 });
    expect(calculator("$1")).toBe("100");
    expect(calculator("EUR1")).toBe("120");
    expect(calculator("$1 + EUR1")).toBe("220");
  });
});

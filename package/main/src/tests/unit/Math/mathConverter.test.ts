import { mathConverter } from "@/Math/mathConverter";

describe("mathConverter function", () => {
  // Test multiplication conversion
  test("should convert multiplication", () => {
    expect(mathConverter("1250*1250")).toBe("1500*1000+400*100+200*100+50*50");
    expect(mathConverter("1350*1350")).toBe(
      "1700*1000+600*100+400*100+200*100+50*50",
    );
    expect(mathConverter("1550*1550")).toBe(
      "2100*1000+1000*100+800*100+600*100+400*100+200*100+50*50",
    );
  });

  // Test exponentiation conversion
  test("should convert exponentiation", () => {
    expect(mathConverter("1250^2")).toBe("1500*1000+400*100+200*100+50*50");
    expect(mathConverter("1350^2")).toBe(
      "1700*1000+600*100+400*100+200*100+50*50",
    );
  });

  // Test cases where no conversion is needed
  test("should not convert when unnecessary", () => {
    expect(mathConverter("1250")).toBe("1250");
  });

  // Test cases with multiple operands
  test("should handle multiple operands", () => {
    expect(mathConverter("1250+1250")).toBe("1250+1250");
  });

  // Test invalid inputs
  test("should handle invalid input", () => {
    expect(mathConverter("abc")).toBe("abc");
  });

  // Test when primary value is missing
  test("should handle missing primary value", () => {
    expect(mathConverter("0*0")).toBe("0*0");
  });

  test("should stop once an expansion repeats", () => {
    expect(mathConverter("5*5")).toBe("5*5+0*0");
    expect(mathConverter("10*10")).toBe("10*10+0*0");
    expect(mathConverter("100*100")).toBe("100*100+0*0");
    expect(mathConverter("10^3")).toBe("10*10+0*0");
    expect(mathConverter("2000*2000")).toBe("3000*1000+1000*1000+0*0");
  });
});

import { gcd } from "@/Math/gcd";
describe("gcd", () => {
  test("gcd of 0 and 0 is 0", () => {
    expect(gcd(0, 0)).toBe(0);
  });

  test("gcd of 0 and non-zero number is 0", () => {
    expect(gcd(0, 5)).toBe(0);
  });

  test("gcd of two numbers", () => {
    expect(gcd(12, 18)).toBe(6);
  });

  test("gcd of three numbers", () => {
    expect(gcd(12, 18, 24)).toBe(6);
  });

  test("gcd of four numbers", () => {
    expect(gcd(12, 18, 24, 36)).toBe(6);
  });
});

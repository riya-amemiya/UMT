import { prime } from "@/Validate/number/prime";

describe("prime", () => {
  it("should return true for prime numbers", () => {
    const validatePrime = prime();
    expect(validatePrime.validate(2)).toBe(true);
    expect(validatePrime.validate(3)).toBe(true);
    expect(validatePrime.validate(5)).toBe(true);
    expect(validatePrime.validate(7)).toBe(true);
  });

  it("should return false for non-prime numbers", () => {
    const validatePrime = prime();
    expect(validatePrime.validate(4)).toBe(false);
    expect(validatePrime.validate(6)).toBe(false);
    expect(validatePrime.validate(8)).toBe(false);
    expect(validatePrime.validate(9)).toBe(false);
  });

  it("should return the custom message for invalid input", () => {
    const customMessage = "This is not a prime number";
    const validatePrime = prime(customMessage);
    expect(validatePrime.validate(4)).toBe(false);
    expect(validatePrime.message).toBe(customMessage);
  });
});

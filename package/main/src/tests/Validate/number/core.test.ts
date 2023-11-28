import { number } from "@/Validate/number/core";
import { maxValue } from "@/Validate/number/maxValue";
import { minValue } from "@/Validate/number/minValue";

describe("number function", () => {
  it("should validate number with no additional options", () => {
    const validateNumber = number([]);
    expect(validateNumber(5).validate).toBeTruthy();
    // @ts-ignore
    expect(validateNumber("5").validate).toBeFalsy();
  });

  it("should validate number with max value option", () => {
    const validateNumberMax = number([maxValue(10)]);
    expect(validateNumberMax(5).validate).toBeTruthy();
    expect(validateNumberMax(11).validate).toBeFalsy();
  });

  it("should validate number with min value option", () => {
    const validateNumberMin = number([minValue(3)]);
    expect(validateNumberMin(5).validate).toBeTruthy();
    expect(validateNumberMin(2).validate).toBeFalsy();
  });

  it("should validate number with both min and max value options", () => {
    const validateNumberMinMax = number([minValue(3), maxValue(10)]);
    expect(validateNumberMinMax(5).validate).toBeTruthy();
    expect(validateNumberMinMax(2).validate).toBeFalsy();
    expect(validateNumberMinMax(11).validate).toBeFalsy();
  });

  it("should return custom message on validation failure", () => {
    const customMessage = "Value must be between 3 and 10";
    const validateNumberMinMax = number([
      minValue(3, customMessage),
      maxValue(10),
    ]);
    const result = validateNumberMinMax(2);
    expect(result.validate).toBeFalsy();
    expect(result.message).toBe(customMessage);
  });
});

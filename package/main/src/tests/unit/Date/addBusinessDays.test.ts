import { addBusinessDays } from "@/Date/addBusinessDays";

describe("addBusinessDays", () => {
  it("adds one business day from Friday to Monday", () => {
    const friday = new Date(2025, 3, 18, 9, 30);
    const result = addBusinessDays(friday, 1);
    expect(result.getFullYear()).toBe(2025);
    expect(result.getMonth()).toBe(3);
    expect(result.getDate()).toBe(21);
    expect(result.getHours()).toBe(9);
    expect(result.getMinutes()).toBe(30);
  });

  it("adds one business day from Saturday to Monday", () => {
    const result = addBusinessDays(new Date(2025, 3, 19), 1);
    expect(result.getDate()).toBe(21);
  });

  it("subtracts one business day from Monday to Friday", () => {
    const result = addBusinessDays(new Date(2025, 3, 21), -1);
    expect(result.getDate()).toBe(18);
  });

  it("returns a clone when amount is 0", () => {
    const saturday = new Date(2025, 3, 19, 12, 0);
    const result = addBusinessDays(saturday, 0);
    expect(result.getTime()).toBe(saturday.getTime());
    expect(result).not.toBe(saturday);
  });

  it("skips holidays when walking forward", () => {
    const monday = new Date(2025, 3, 21);
    const holidays = [new Date(2025, 3, 22)];
    const result = addBusinessDays(monday, 1, holidays);
    expect(result.getDate()).toBe(23);
  });

  it("skips holidays when walking backward", () => {
    const wednesday = new Date(2025, 3, 23);
    const holidays = [new Date(2025, 3, 22)];
    const result = addBusinessDays(wednesday, -1, holidays);
    expect(result.getDate()).toBe(21);
  });

  it("does not mutate the input date", () => {
    const friday = new Date(2025, 3, 18);
    const original = friday.getTime();
    addBusinessDays(friday, 1);
    expect(friday.getTime()).toBe(original);
  });
});

import { addBusinessDays } from "@/Date/addBusinessDays";
import { subBusinessDays } from "@/Date/subBusinessDays";

describe("subBusinessDays", () => {
  it("subtracts one business day from Monday to Friday", () => {
    const result = subBusinessDays(new Date(2025, 3, 21, 9, 30), 1);
    expect(result.getDate()).toBe(18);
    expect(result.getHours()).toBe(9);
    expect(result.getMinutes()).toBe(30);
  });

  it("matches addBusinessDays with a negated amount", () => {
    const date = new Date(2025, 3, 21);
    const holidays = [new Date(2025, 3, 18)];
    expect(subBusinessDays(date, 2, holidays).getTime()).toBe(
      addBusinessDays(date, -2, holidays).getTime(),
    );
  });

  it("returns a clone when amount is 0", () => {
    const date = new Date(2025, 3, 21, 8, 0);
    const result = subBusinessDays(date, 0);
    expect(result.getTime()).toBe(date.getTime());
    expect(result).not.toBe(date);
  });
});

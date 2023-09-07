import { dayOfWeekSimple } from "@/Simple/Tool/dayOfWeekSimple";
describe("dayOfWeekSimple", () => {
  it("should return 0 for Sunday", () => {
    expect(dayOfWeekSimple("2022-01-02")).toBe(0);
  });

  it("should return 1 for Monday", () => {
    expect(dayOfWeekSimple("2022-01-03")).toBe(1);
  });

  it("should return 2 for Tuesday", () => {
    expect(dayOfWeekSimple("2022-01-04")).toBe(2);
  });

  it("should return 3 for Wednesday", () => {
    expect(dayOfWeekSimple("2022-01-05")).toBe(3);
  });

  it("should return 4 for Thursday", () => {
    expect(dayOfWeekSimple("2022-01-06")).toBe(4);
  });

  it("should return 5 for Friday", () => {
    expect(dayOfWeekSimple("2022-01-07")).toBe(5);
  });

  it("should return 6 for Saturday", () => {
    expect(dayOfWeekSimple("2022-01-08")).toBe(6);
  });

  it("should return 0 for Sunday (Date)", () => {
    const date = new Date("2022-01-02");
    expect(dayOfWeekSimple(date)).toBe(0);
  });

  it("should return 1 for Monday (Date)", () => {
    const date = new Date("2022-01-03");
    expect(dayOfWeekSimple(date)).toBe(1);
  });

  it("should return 2 for Tuesday (Date)", () => {
    const date = new Date("2022-01-04");
    expect(dayOfWeekSimple(date)).toBe(2);
  });

  it("should return 3 for Wednesday", () => {
    const date = new Date("2022-01-05");
    expect(dayOfWeekSimple(date)).toBe(3);
  });

  it("should return 4 for Thursday (Date)", () => {
    const date = new Date("2022-01-06");
    expect(dayOfWeekSimple(date)).toBe(4);
  });

  it("should return 5 for Friday (Date)", () => {
    const date = new Date("2022-01-07");
    expect(dayOfWeekSimple(date)).toBe(5);
  });

  it("should return 6 for Saturday (Date)", () => {
    const date = new Date("2022-01-08");
    expect(dayOfWeekSimple(date)).toBe(6);
  });
});

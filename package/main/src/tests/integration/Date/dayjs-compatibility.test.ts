import dayjs from "dayjs";
import isBetweenPlugin from "dayjs/plugin/isBetween";
import quarterOfYear from "dayjs/plugin/quarterOfYear";

import { addDuration } from "@/Date/addDuration";
import { diff } from "@/Date/diff";
import type { DurationUnit } from "@/Date/durationUnit";
import { endOf } from "@/Date/endOf";
import { format } from "@/Date/format";
import { fromUnix } from "@/Date/fromUnix";
import { getQuarter } from "@/Date/getQuarter";
import { isBetween } from "@/Date/isBetween";
import { isSame } from "@/Date/isSame";
import { isSameDay } from "@/Date/isSameDay";
import { isWeekend } from "@/Date/isWeekend";
import type { DateBoundaryUnit } from "@/Date/startOf";
import { startOf } from "@/Date/startOf";
import { subDuration } from "@/Date/subDuration";
import { toUnix } from "@/Date/toUnix";

dayjs.extend(isBetweenPlugin);
dayjs.extend(quarterOfYear);

const durationUnitToDayjs: Record<
  DurationUnit,
  dayjs.ManipulateType | "millisecond"
> = {
  ms: "millisecond",
  s: "second",
  m: "minute",
  h: "hour",
  d: "day",
  w: "week",
  M: "month",
  y: "year",
};

const boundaryUnits: DateBoundaryUnit[] = [
  "second",
  "minute",
  "hour",
  "day",
  "week",
  "month",
  "quarter",
  "year",
];

const dayjsIsSame = (
  left: Date,
  right: Date,
  unit: DateBoundaryUnit,
): boolean => {
  if (unit === "quarter") {
    return dayjs(left).isSame(right, "quarter");
  }
  return dayjs(left).isSame(right, unit);
};

describe("dayjs compatibility for Date utilities", () => {
  describe("isSame", () => {
    const left = new Date(2025, 3, 15, 10, 30, 45, 123);
    const sameDay = new Date(2025, 3, 15, 23, 59, 59, 999);
    const nextDay = new Date(2025, 3, 16, 0, 0, 0, 0);
    const sameWeek = new Date(2025, 3, 19);
    const nextWeek = new Date(2025, 3, 20);
    const sameQuarter = new Date(2025, 5, 30);
    const nextQuarter = new Date(2025, 6, 1);

    it("matches exact equality without unit", () => {
      const clone = new Date(left);
      expect(isSame(left, clone)).toBe(dayjs(left).isSame(clone));
      expect(isSame(left, sameDay)).toBe(dayjs(left).isSame(sameDay));
    });

    it.each(boundaryUnits)("matches unit granularity for %s", (unit) => {
      const pairs: [Date, Date][] = [
        [left, sameDay],
        [left, nextDay],
        [left, sameWeek],
        [left, nextWeek],
        [left, sameQuarter],
        [left, nextQuarter],
        [new Date(2025, 0, 1), new Date(2025, 11, 31)],
        [new Date(2024, 11, 31), new Date(2025, 0, 1)],
      ];
      for (const [a, b] of pairs) {
        expect(isSame(a, b, unit)).toBe(dayjsIsSame(a, b, unit));
      }
    });
  });

  describe("isSameDay", () => {
    it("matches dayjs isSame with day unit", () => {
      const cases: [Date, Date][] = [
        [new Date(2025, 3, 15, 1), new Date(2025, 3, 15, 23)],
        [new Date(2025, 3, 15), new Date(2025, 3, 16)],
        [new Date(2025, 3, 15), new Date(2025, 4, 15)],
        [new Date(2024, 3, 15), new Date(2025, 3, 15)],
      ];
      for (const [left, right] of cases) {
        expect(isSameDay(left, right)).toBe(dayjs(left).isSame(right, "day"));
        expect(isSame(left, right, "day")).toBe(
          dayjs(left).isSame(right, "day"),
        );
      }
    });
  });

  describe("startOf", () => {
    const sample = new Date(2025, 4, 15, 10, 30, 45, 123);

    it.each(boundaryUnits)("matches dayjs startOf for %s", (unit) => {
      expect(startOf(sample, unit).getTime()).toBe(
        dayjs(sample).startOf(unit).valueOf(),
      );
    });

    it("matches Sunday week start for mid-week date", () => {
      const wednesday = new Date(2025, 3, 16, 12, 0, 0, 0);
      expect(startOf(wednesday, "week").getTime()).toBe(
        dayjs(wednesday).startOf("week").valueOf(),
      );
      expect(startOf(wednesday, "week").getDay()).toBe(0);
    });
  });

  describe("endOf", () => {
    const sample = new Date(2025, 4, 15, 10, 30, 45, 123);

    it.each(boundaryUnits)("matches dayjs endOf for %s", (unit) => {
      expect(endOf(sample, unit).getTime()).toBe(
        dayjs(sample).endOf(unit).valueOf(),
      );
    });
  });

  describe("addDuration / subDuration", () => {
    const base = new Date(2025, 0, 15, 12, 30, 0, 0);
    const amounts = [0, 1, 2, 7, -1, -3];

    it.each(Object.keys(durationUnitToDayjs) as DurationUnit[])(
      "matches dayjs add for unit %s",
      (unit) => {
        for (const amount of amounts) {
          const dayjsUnit = durationUnitToDayjs[unit];
          expect(addDuration(base, amount, unit).getTime()).toBe(
            dayjs(base).add(amount, dayjsUnit).valueOf(),
          );
          expect(subDuration(base, amount, unit).getTime()).toBe(
            dayjs(base).subtract(amount, dayjsUnit).valueOf(),
          );
        }
      },
    );

    it("matches month-end clamping for January 31", () => {
      const jan31 = new Date(2025, 0, 31, 0, 0, 0, 0);
      expect(addDuration(jan31, 1, "M").getTime()).toBe(
        dayjs(jan31).add(1, "month").valueOf(),
      );
      expect(subDuration(new Date(2025, 2, 31), 1, "M").getTime()).toBe(
        dayjs(new Date(2025, 2, 31))
          .subtract(1, "month")
          .valueOf(),
      );
    });
  });

  describe("diff", () => {
    const cases: [Date, Date, DurationUnit][] = [
      [new Date("2025-12-31"), new Date("2025-01-01"), "d"],
      [new Date("2025-01-01T05:00:00Z"), new Date("2025-01-01T00:00:00Z"), "h"],
      [new Date("2025-01-01T00:30:00Z"), new Date("2025-01-01T00:00:00Z"), "m"],
      [new Date("2025-01-01T00:00:30Z"), new Date("2025-01-01T00:00:00Z"), "s"],
      [
        new Date("2025-01-01T00:00:00.500Z"),
        new Date("2025-01-01T00:00:00Z"),
        "ms",
      ],
      [new Date("2025-01-15"), new Date("2025-01-01"), "w"],
      [new Date(2025, 2, 14), new Date(2025, 0, 15), "M"],
      [new Date(2025, 2, 16), new Date(2025, 0, 15), "M"],
      [new Date(2025, 0, 15), new Date(2025, 2, 14), "M"],
      [new Date(2026, 0, 1), new Date(2025, 0, 1), "y"],
      [
        new Date(2025, 1, 15, 10, 0, 0, 0),
        new Date(2025, 0, 15, 11, 0, 0, 0),
        "M",
      ],
    ];

    it.each(cases)(
      "matches dayjs.diff for %p - %p in %s",
      (left, right, unit) => {
        const dayjsUnit = durationUnitToDayjs[unit];
        expect(diff(left, right, unit)).toBe(
          dayjs(left).diff(right, dayjsUnit),
        );
      },
    );
  });

  describe("format", () => {
    const sample = new Date(2025, 3, 15, 15, 30, 45, 123);
    const patterns = [
      "YYYY-MM-DD",
      "YYYY-MM-DD HH:mm:ss",
      "MM/DD/YYYY",
      "HH:mm",
      "hh:mm A",
      "YYYY-MM-DDTHH:mm:ssZ",
      "YYYY-MM-DDTHH:mm:ssZZ",
      "YY-M-D H:m:s",
      "SSS",
      "d",
    ];

    it.each(patterns)("matches dayjs format for pattern %s", (pattern) => {
      expect(format(sample, pattern)).toBe(dayjs(sample).format(pattern));
    });
  });

  describe("isWeekend", () => {
    it("matches Saturday and Sunday detection via dayjs day()", () => {
      const dates = [
        new Date(2025, 3, 18),
        new Date(2025, 3, 19),
        new Date(2025, 3, 20),
        new Date(2025, 3, 21),
      ];
      for (const date of dates) {
        const day = dayjs(date).day();
        expect(isWeekend(date)).toBe(day === 0 || day === 6);
      }
    });
  });

  describe("isBetween", () => {
    const start = new Date(2025, 3, 10, 10, 0, 0, 0);
    const mid = new Date(2025, 3, 15, 12, 0, 0, 0);
    const end = new Date(2025, 3, 20, 18, 0, 0, 0);
    const inclusivities = ["()", "[]", "[)", "(]"] as const;

    it("matches exclusive default bounds", () => {
      expect(isBetween(mid, start, end)).toBe(dayjs(mid).isBetween(start, end));
      expect(isBetween(start, start, end)).toBe(
        dayjs(start).isBetween(start, end),
      );
    });

    it.each(inclusivities)("matches inclusivity %s", (inclusivity) => {
      expect(isBetween(start, start, end, undefined, inclusivity)).toBe(
        dayjs(start).isBetween(start, end, null, inclusivity),
      );
      expect(isBetween(end, start, end, undefined, inclusivity)).toBe(
        dayjs(end).isBetween(start, end, null, inclusivity),
      );
    });

    it.each(boundaryUnits)("matches unit granularity for %s", (unit) => {
      const dayjsUnit = unit === "quarter" ? "quarter" : unit;
      expect(isBetween(mid, start, end, unit, "[]")).toBe(
        dayjs(mid).isBetween(start, end, dayjsUnit as "day", "[]"),
      );
    });
  });

  describe("getQuarter", () => {
    it("matches dayjs quarter", () => {
      const dates = [
        new Date(2025, 0, 1),
        new Date(2025, 3, 15),
        new Date(2025, 6, 1),
        new Date(2025, 11, 31),
      ];
      for (const date of dates) {
        expect(getQuarter(date)).toBe(dayjs(date).quarter());
      }
    });
  });

  describe("fromUnix / toUnix", () => {
    it("matches dayjs.unix for seconds", () => {
      const seconds = 1_700_000_000;
      expect(fromUnix(seconds).getTime()).toBe(dayjs.unix(seconds).valueOf());
      expect(toUnix(fromUnix(seconds))).toBe(dayjs.unix(seconds).unix());
    });

    it("matches dayjs valueOf for millisecond unit", () => {
      const date = new Date(1_700_000_000_123);
      expect(toUnix(date, "ms")).toBe(dayjs(date).valueOf());
    });
  });
});

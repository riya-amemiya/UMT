import type { ArrayToUnion } from "$/logicType";

interface DayList {
  de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
  ko: ["일", "월", "화", "수", "목", "금", "토"];
  en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  ja: ["日", "月", "火", "水", "木", "金", "土"];
}

/**
 * 数値を曜日に変換する
 * @param day 曜日を表す数値
 * @param lang 言語
 * @returns langに応じた曜日
 * @example getDay(0); // "日"
 */
function getDay(day: number, lang: "de"): ArrayToUnion<DayList["de"]>;
function getDay(day: number, lang: "ko"): ArrayToUnion<DayList["ko"]>;
function getDay(day: number, lang: "en"): ArrayToUnion<DayList["en"]>;
function getDay(day: number, lang: "ja"): ArrayToUnion<DayList["ja"]>;
function getDay(day: number): ArrayToUnion<DayList["ja"]>;
function getDay(day: number, lang: "de" | "ko" | "en" | "ja" = "ja") {
  const dayList: DayList = {
    de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    ko: ["일", "월", "화", "수", "목", "금", "토"],
    en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    ja: ["日", "月", "火", "水", "木", "金", "土"],
  };
  switch (day) {
    case 0: {
      return dayList[lang][0];
    }
    case 1: {
      return dayList[lang][1];
    }
    case 2: {
      return dayList[lang][2];
    }
    case 3: {
      return dayList[lang][3];
    }
    case 4: {
      return dayList[lang][4];
    }
    case 5: {
      return dayList[lang][5];
    }
    case 6: {
      return dayList[lang][6];
    }
    default: {
      return dayList[lang][0];
    }
  }
}

export { getDay };

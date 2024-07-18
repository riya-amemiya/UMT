import type { ArrayToUnion } from "$/logic/arrayToUnion";

interface DayList {
  de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
  ko: ["일", "월", "화", "수", "목", "금", "토"];
  en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  ja: ["日", "月", "火", "水", "木", "金", "土"];
  fr: ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"];
}

/**
 * 数値を曜日に変換する
 * @param day 曜日を表す数値
 * @param lang 言語
 * @returns langに応じた曜日
 * @example getDay(0); // "日"
 */
export const getDay = <T extends keyof DayList>(
  day: number,
  lang: T = "ja" as T,
): ArrayToUnion<DayList[T]> => {
  const dayList: DayList = {
    de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    ko: ["일", "월", "화", "수", "목", "금", "토"],
    en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    ja: ["日", "月", "火", "水", "木", "金", "土"],
    fr: ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"],
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
};

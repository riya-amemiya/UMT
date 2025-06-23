import type { ArrayToUnion } from "$/logic/arrayToUnion";

interface DayList {
  de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
  ko: ["일", "월", "화", "수", "목", "금", "토"];
  en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
  ja: ["日", "月", "火", "水", "木", "金", "土"];
  fr: ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"];
}

/**
 * Convert a number to a day of the week in the specified language
 * @param day Number representing the day (0-6, where 0 is Sunday)
 * @param lang Language code ('de', 'ko', 'en', 'ja', 'fr')
 * @returns Day of the week string in the specified language
 * @example
 * getDay(0); // Returns "日" (default is Japanese)
 * getDay(0, "en"); // Returns "Sun"
 * getDay(1, "fr"); // Returns "Lun"
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

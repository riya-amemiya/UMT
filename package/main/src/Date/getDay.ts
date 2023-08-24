function getDay(
  day: number,
  lang: "de",
): "So" | "Mo" | "Di" | "Mi" | "Do" | "Fr" | "Sa";
function getDay(
  day: number,
  lang: "ko",
): "일" | "월" | "화" | "수" | "목" | "금" | "토";
function getDay(
  day: number,
  lang: "en",
): "Sun" | "Mon" | "Tue" | "Wed" | "Thu" | "Fri" | "Sat";
function getDay(
  day: number,
  lang: "ja",
): "日" | "月" | "火" | "水" | "木" | "金" | "土";
function getDay(day: number): "日" | "月" | "火" | "水" | "木" | "金" | "土";
function getDay(day: number, lang: "de" | "ko" | "en" | "ja" = "ja") {
  const dayList: {
    [key in typeof lang]: [
      string,
      string,
      string,
      string,
      string,
      string,
      string,
    ];
  } = {
    de: ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"],
    ko: ["일", "월", "화", "수", "목", "금", "토"],
    en: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    ja: ["日", "月", "火", "水", "木", "金", "土"],
  };
  switch (day) {
    case 0:
      return dayList[lang][0];
    case 1:
      return dayList[lang][1];
    case 2:
      return dayList[lang][2];
    case 3:
      return dayList[lang][3];
    case 4:
      return dayList[lang][4];
    case 5:
      return dayList[lang][5];
    case 6:
      return dayList[lang][6];
    default:
      return dayList[lang][0];
  }
}

export { getDay };

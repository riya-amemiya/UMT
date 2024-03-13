/**
 * 1秒のミリ秒数
 */
export const OneSecondMs = 1000 as const;

/**
 * 1分のミリ秒数
 */
export const OneMinuteMs = (OneSecondMs * 60) as 60_000;

/**
 * 1時間のミリ秒数
 */
export const OneHourMs = (OneMinuteMs * 60) as 3_600_000;

/**
 * 1日のミリ秒数
 */
export const OneDayMs = (OneHourMs * 24) as 86_400_000;

/**
 * 1週間のミリ秒数
 */
export const OneWeekMs = (OneDayMs * 7) as 604_800_000;

/**
 * 1ヶ月のミリ秒数(28日)
 */
export const OneMonthMs28 = (OneDayMs * 28) as 2_419_200_000;

/**
 * 1ヶ月のミリ秒数(29日)
 */
export const OneMonthMs29 = (OneDayMs * 29) as 2_505_600_000;

/**
 * 1ヶ月のミリ秒数(30日)
 */
export const OneMonthMs = (OneDayMs * 30) as 2_592_000_000;

/**
 * 1ヶ月のミリ秒数(31日)
 */
export const OneMonthMs31 = (OneDayMs * 31) as 2_678_400_000;

/**
 * 1年のミリ秒数(365日)
 */
export const OneYearMs = (OneDayMs * 365) as 31_536_000_000;

/**
 * 1年のミリ秒数(366日)
 */
export const OneYearMs366 = (OneDayMs * 366) as 31_622_400_000;

/**
 * Number of milliseconds in one second
 */
export const OneSecondMs = 1000 as const;

/**
 * Number of milliseconds in one minute
 */
export const OneMinuteMs = (OneSecondMs * 60) as 60_000;

/**
 * Number of milliseconds in one hour
 */
export const OneHourMs = (OneMinuteMs * 60) as 3_600_000;

/**
 * Number of milliseconds in one day
 */
export const OneDayMs = (OneHourMs * 24) as 86_400_000;

/**
 * Number of milliseconds in one week
 */
export const OneWeekMs = (OneDayMs * 7) as 604_800_000;

/**
 * Number of milliseconds in one month (28 days)
 */
export const OneMonthMs28 = (OneDayMs * 28) as 2_419_200_000;

/**
 * Number of milliseconds in one month (29 days)
 */
export const OneMonthMs29 = (OneDayMs * 29) as 2_505_600_000;

/**
 * Number of milliseconds in one month (30 days)
 */
export const OneMonthMs = (OneDayMs * 30) as 2_592_000_000;

/**
 * Number of milliseconds in one month (31 days)
 */
export const OneMonthMs31 = (OneDayMs * 31) as 2_678_400_000;

/**
 * Number of milliseconds in one year (365 days)
 */
export const OneYearMs = (OneDayMs * 365) as 31_536_000_000;

/**
 * Number of milliseconds in one year (366 days)
 */
export const OneYearMs366 = (OneDayMs * 366) as 31_622_400_000;

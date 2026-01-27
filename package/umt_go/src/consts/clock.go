package consts

// Clock constants define common time durations in milliseconds.

// OneSecondMs is the number of milliseconds in one second.
const OneSecondMs = 1000

// OneMinuteMs is the number of milliseconds in one minute.
const OneMinuteMs = OneSecondMs * 60 // 60_000

// OneHourMs is the number of milliseconds in one hour.
const OneHourMs = OneMinuteMs * 60 // 3_600_000

// OneDayMs is the number of milliseconds in one day.
const OneDayMs = OneHourMs * 24 // 86_400_000

// OneWeekMs is the number of milliseconds in one week.
const OneWeekMs = OneDayMs * 7 // 604_800_000

// OneMonthMs28 is the number of milliseconds in one month (28 days).
const OneMonthMs28 = OneDayMs * 28 // 2_419_200_000

// OneMonthMs29 is the number of milliseconds in one month (29 days).
const OneMonthMs29 = OneDayMs * 29 // 2_505_600_000

// OneMonthMs is the number of milliseconds in one month (30 days).
const OneMonthMs = OneDayMs * 30 // 2_592_000_000

// OneMonthMs31 is the number of milliseconds in one month (31 days).
const OneMonthMs31 = OneDayMs * 31 // 2_678_400_000

// OneYearMs is the number of milliseconds in one year (365 days).
const OneYearMs = OneDayMs * 365 // 31_536_000_000

// OneYearMs366 is the number of milliseconds in one year (366 days, leap year).
const OneYearMs366 = OneDayMs * 366 // 31_622_400_000

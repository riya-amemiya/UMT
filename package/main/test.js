const { DateWrapper } = require("./module/Date/DateWrapper");
const date1 = new Date("2021-01-01T09:30");
const date2 = new DateWrapper(date1);
console.log("====================================");
console.log("date1", date1);
console.log("date1.hours", date1.getHours());
console.log("date1.UTC.hours", date1.getUTCHours());
console.log("====================================");

console.log("====================================");
console.log("date2", date2);
console.log("date2.hours", date2.getHour());
console.log("====================================");

const { DateWrapper } = require("./module/Date/DateWrapper");
const date1 = new Date("2021-01-01T10:30");
const date2 = new DateWrapper(date1);
const date3 = new Date("2021-01-01");
const date4 = new DateWrapper(date3);
console.log("====================================");
console.log("date1", date1.getTimezoneOffset());
console.log("date1.hours", date1.getHours());
console.log("date1.UTC.hours", date1.getUTCHours());
console.log("====================================");

console.log("====================================");
console.log("date2", date2);
console.log("date2.hours", date2.getHours());
console.log("====================================");

console.log("====================================");
console.log("date3", date3);
console.log("date3.hours", date3.getHours());
console.log("date3.UTC.hours", date3.getUTCHours());
console.log("====================================");

console.log("====================================");
console.log("date4", date4);
console.log("date4.hours", date4.getHours());
console.log("====================================");

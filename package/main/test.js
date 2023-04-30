const { newDateStr, newDateInt } = require("./module/Date/new");
const { now } = require("./module/Date/now");
const { DateWrapper } = require("./module/Date/DateWrapper");
const date1 = newDateStr("1995-02-01");
const date2 = newDateInt(1995, 2, 1);
const date3 = new DateWrapper();
const n = now();
const n2 = new Date();

console.log(new Date().getTimezoneOffset());
console.log("====================================");
console.log(n);
console.log(n.getUTCHours());
console.log(n.getHours());
console.log(n.getUTCDate());
console.log(n.toString());
console.log("====================================");
console.log(n2);
console.log(n2.getUTCHours());
console.log(n2.getHours());
console.log(n2.getUTCDate());
console.log(n2.toString());
console.log("====================================");
console.log(date1);
console.log(date1.getUTCHours());
console.log(date1.getHours());
console.log(date1.getUTCDate());
console.log(date1.toString());
console.log("====================================");
console.log(date2);
console.log(date2.getUTCHours());
console.log(date2.getHours());
console.log(date2.getUTCDate());
console.log(date2.toString());
console.log("====================================");
console.log(date3.date.toJSON("yyyy"));
console.log(date3.setTimeDifference(9).setNow().subMonth(1).getDateObj());
console.log("====================================");

export type monTypeZero =
	| `${0}${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
	| `${1}${0 | 1 | 2}`;
export type monTypeNoZero =
	| `${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
	| `${1}${0 | 1 | 2}`;
export type monType = monTypeZero | monTypeNoZero;
export type monTypeInt = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
export type MonthsWith31Days =
	| `${0}${1 | 3 | 5 | 7 | 8}`
	| `${1}${0 | 2}`
	| `${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`;
export type MonthsWith31DaysInt = 1 | 3 | 5 | 7 | 8 | 10 | 12;
export type MonthsWihout31Days =
	| `${0}${2 | 4 | 6 | 9}`
	| `${1}${1}`
	| `${2 | 4 | 6 | 9}`;
export type MonthsWihout31DaysInt = 2 | 4 | 6 | 9 | 11;
export type dayType<T extends string> = T extends "02" | "2"
	?
			| `${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${0}${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${1}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${2}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
	:
			| `${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${0}${1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${1}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${2}${0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9}`
			| `${3}${T extends MonthsWihout31Days ? 0 : 0 | 1}`;

export type dayTypeInt<T extends number> = T extends 2
	?
			| 1
			| 2
			| 3
			| 4
			| 5
			| 6
			| 7
			| 8
			| 9
			| 10
			| 11
			| 12
			| 13
			| 14
			| 15
			| 16
			| 17
			| 18
			| 19
			| 20
			| 21
			| 22
			| 23
			| 24
			| 25
			| 26
			| 27
			| 28
			| 29
	: T extends MonthsWihout31DaysInt
	?
			| 1
			| 2
			| 3
			| 4
			| 5
			| 6
			| 7
			| 8
			| 9
			| 10
			| 11
			| 12
			| 13
			| 14
			| 15
			| 16
			| 17
			| 18
			| 19
			| 20
			| 21
			| 22
			| 23
			| 24
			| 25
			| 26
			| 27
			| 28
			| 29
			| 30
	:
			| 1
			| 2
			| 3
			| 4
			| 5
			| 6
			| 7
			| 8
			| 9
			| 10
			| 11
			| 12
			| 13
			| 14
			| 15
			| 16
			| 17
			| 18
			| 19
			| 20
			| 21
			| 22
			| 23
			| 24
			| 25
			| 26
			| 27
			| 28
			| 29
			| 30
			| 31;

export type DateType<T extends monType> =
	| `${number}-${monType}-${dayType<T>}`
	| `${number}/${monType}/${dayType<T>}`
	| `${number}.${monType}.${dayType<T>}`;

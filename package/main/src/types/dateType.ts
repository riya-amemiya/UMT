import { int } from "./int";
import { dayType, monType } from "./monType";
export type millisecondsType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type secondsType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type minutesType = `${0 | 1 | 2 | 3 | 4 | 5}${int}` | "60";
export type hoursType = `${0 | 1}${int}` | "24";
export type DateType =
	| `${number}-${monType}-${dayType<monType>}`
	| `${number}/${monType}/${dayType<monType>}`
	| `${number}.${monType}.${dayType<monType>}`
	| `${number}-${monType}-${dayType<monType>}T${number}:${number}:${number}.${number}Z`
	| `${number}-${monType}-${dayType<monType>}T${number}:${number}:${number}.${number}+${number}:${number}`
	| `${number}-${monType}-${dayType<monType>}T${number}:${number}:${number}.${number}-${number}:${number}`
	| `${number}/${monType}/${dayType<monType>}T${number}:${number}:${number}.${number}Z`
	| `${number}/${monType}/${dayType<monType>}T${number}:${number}:${number}.${number}+${number}:${number}`
	| `${number}/${monType}/${dayType<monType>}T${number}:${number}:${number}.${number}-${number}:${number}`
	| `${number}.${monType}.${dayType<monType>}T${number}:${number}:${number}.${number}Z`
	| `${number}.${monType}.${dayType<monType>}T${number}:${number}:${number}.${number}+${number}:${number}`
	| `${number}.${monType}.${dayType<monType>}T${number}:${number}:${number}.${number}-${number}:${number}`;

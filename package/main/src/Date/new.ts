import { DateType } from "../types/dateType";

export const newDate = (dateString: DateType): Date => {
	const date = new Date(dateString);
	return date;
};

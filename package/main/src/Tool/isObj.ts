// rome-ignore lint/suspicious/noExplicitAny: <explanation>
export const isObj = <T>(obj: any): obj is T => {
	return typeof obj === "object" && obj !== null;
};

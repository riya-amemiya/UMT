// Type to check if type is 'any'
export type IsAny<T> = 0 extends 1 & T ? true : false;

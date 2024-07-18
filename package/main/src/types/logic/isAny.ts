// any型かどうかを判定する型
export type IsAny<T> = 0 extends 1 & T ? true : false;

// Array型をUnion型に変換する型
export type ArrayToUnion<X extends unknown[]> = X[number];

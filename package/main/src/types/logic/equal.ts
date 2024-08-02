// XとYが等しいかどうかを判定する型
export type Equal<X, Y> = X extends Y ? true : false;

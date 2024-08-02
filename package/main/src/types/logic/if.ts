// if文の型
export type IF<C extends boolean, X, Y> = C extends true ? X : Y;

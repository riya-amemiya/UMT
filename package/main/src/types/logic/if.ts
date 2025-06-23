// Type for conditional (if) operation
export type IF<C extends boolean, X, Y> = C extends true ? X : Y;

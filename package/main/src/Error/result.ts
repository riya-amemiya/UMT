interface OkType<T> {
  type: "ok";
  value: T;
}

interface ErrorType<E> {
  type: "err";
  error: E;
}

export type Result<T, E> = OkType<T> | ErrorType<E>;
const okFunction = <T>(value: T): OkType<T> => ({ type: "ok", value });
const errorFunction = <T>(error: T): ErrorType<T> => ({ type: "err", error });
export const result = <O, E = Error>(callback: () => O): Result<O, E> => {
  try {
    return okFunction<O>(callback());
  } catch (error) {
    return errorFunction<E>(error);
  }
};

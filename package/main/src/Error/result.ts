interface OkType<O> {
  type: "ok";
  value: O;
}

interface ErrorType<E> {
  type: "err";
  error: E;
}

export type Result<O, E> = OkType<O> | ErrorType<E>;
const okFunction = <O>(value: O): OkType<O> => ({ type: "ok", value });
const errorFunction = <E>(error: E): ErrorType<E> => ({ type: "err", error });
export const result = <O, E = Error>(callback: () => O): Result<O, E> => {
  try {
    return okFunction<O>(callback());
  } catch (error) {
    return errorFunction<E>(error);
  }
};

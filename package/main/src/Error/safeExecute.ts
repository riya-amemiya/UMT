interface SuccessType<V> {
  type: "success";
  value: V;
}

interface ErrorType<E> {
  type: "error";
  error: E;
}

export type Result<V, E> = SuccessType<V> | ErrorType<E>;
const errorFunction = <E>(error: E): ErrorType<E> => ({ type: "error", error });
const successFunction = <V>(value: V): SuccessType<V> => ({
  type: "success",
  value,
});
export const safeExecute = <V, E = Error>(callback: () => V): Result<V, E> => {
  try {
    return successFunction<V>(callback());
  } catch (error) {
    return errorFunction<E>(error as E);
  }
};

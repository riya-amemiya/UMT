/**
 * パイプラインを表すインターフェース。
 * 無引数で呼び出された場合、ストアされている値を返す。
 * 関数が引数として与えられた場合、その関数を適用して新しいPipeインスタンスを生成する。
 */
export interface Pipeline<T> {
  (): T;
  <U>(transformer: (input: T) => U): Pipeline<U>;
}

/**
 * Pipelineインスタンスを生成する関数。
 * @param initialValue 初期値。
 * @returns Pipelineインスタンス。
 */
export const createPipeline: <T>(initialValue: T) => Pipeline<T> = <T>(
  initialValue: T,
) =>
  // 引数として変換関数（transformer）を受け取り、新しいPipelineを返す。
  // 引数が無い場合、ストアされた値（initialValue）を返す。
  (<U>(transformer?: (input: T) => U) =>
    transformer
      ? createPipeline(transformer(initialValue))
      : initialValue) as Pipeline<T>;

export interface Pipe<T> {
    (): T;
    <U>(f: (x: T) => U): Pipe<U>;
}
const pipeFunction: <T>(x: T) => Pipe<T> = <T>(x: T) =>
    (<U>(f?: (x: T) => U) => (f ? pipeFunction(f(x)) : x)) as Pipe<T>;
export default pipeFunction;

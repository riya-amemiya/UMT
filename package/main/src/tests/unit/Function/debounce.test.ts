import { debounce } from "@/Function/debounce";

describe("debounce", () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("should delay invocation until after the wait period", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced();
    expect(fn).not.toHaveBeenCalled();

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should reset the timer on subsequent calls", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced();
    jest.advanceTimersByTime(50);
    debounced();
    jest.advanceTimersByTime(50);
    expect(fn).not.toHaveBeenCalled();

    jest.advanceTimersByTime(50);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should pass the latest arguments", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced(1);
    debounced(2);
    debounced(3);

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledWith(3);
  });

  it("should invoke on the leading edge when leading is true", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100, { leading: true, trailing: false });

    debounced();
    expect(fn).toHaveBeenCalledTimes(1);

    debounced();
    expect(fn).toHaveBeenCalledTimes(1);

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should invoke on both leading and trailing edges", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100, { leading: true, trailing: true });

    debounced(1);
    expect(fn).toHaveBeenCalledTimes(1);
    expect(fn).toHaveBeenCalledWith(1);

    debounced(2);
    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(2);
    expect(fn).toHaveBeenLastCalledWith(2);
  });

  it("should cancel pending invocation", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced();
    debounced.cancel();

    jest.advanceTimersByTime(100);
    expect(fn).not.toHaveBeenCalled();
  });

  it("should handle cancel when no timer is pending", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced.cancel();

    jest.advanceTimersByTime(100);
    expect(fn).not.toHaveBeenCalled();
  });

  it("should invoke the pending call immediately on flush", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced(1);
    debounced(2);
    debounced.flush();
    expect(fn).toHaveBeenCalledTimes(1);
    expect(fn).toHaveBeenCalledWith(2);

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should do nothing on flush when no timer is pending", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    debounced.flush();
    expect(fn).not.toHaveBeenCalled();
  });

  it("should not invoke on flush when trailing is false", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100, { leading: true, trailing: false });

    debounced(1);
    debounced(2);
    debounced.flush();
    expect(fn).toHaveBeenCalledTimes(1);
    expect(debounced.pending()).toBe(false);
  });

  it("should not invoke again on flush after the leading call", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100, { leading: true, trailing: true });

    debounced(1);
    debounced.flush();
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should report pending state", () => {
    const fn = jest.fn();
    const debounced = debounce(fn, 100);

    expect(debounced.pending()).toBe(false);
    debounced();
    expect(debounced.pending()).toBe(true);
    jest.advanceTimersByTime(100);
    expect(debounced.pending()).toBe(false);
    debounced();
    debounced.cancel();
    expect(debounced.pending()).toBe(false);
  });
});

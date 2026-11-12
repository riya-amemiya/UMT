import { throttle } from "@/Function/throttle";

describe("throttle", () => {
  beforeEach(() => {
    jest.useFakeTimers();
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  it("should invoke immediately on the first call", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled();
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should throttle subsequent calls within the wait period", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled();
    throttled();
    throttled();

    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should invoke trailing call after the wait period", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled(1);
    throttled(2);

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(2);
    expect(fn).toHaveBeenLastCalledWith(2);
  });

  it("should pass the latest arguments to the trailing call", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled("a");
    throttled("b");
    throttled("c");

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenLastCalledWith("c");
  });

  it("should allow invocation again after wait period has passed", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled();
    jest.advanceTimersByTime(100);

    throttled();
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("should cancel pending invocation", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled();
    throttled();
    throttled.cancel();

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should clear pending timer when called after wait period elapses", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled();
    throttled();

    jest.setSystemTime(Date.now() + 100);

    throttled();
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("should handle cancel when no timer is pending", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled.cancel();
    expect(fn).not.toHaveBeenCalled();
  });

  it("should invoke the pending trailing call immediately on flush", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled(1);
    throttled(2);
    throttled.flush();
    expect(fn).toHaveBeenCalledTimes(2);
    expect(fn).toHaveBeenLastCalledWith(2);

    jest.advanceTimersByTime(100);
    expect(fn).toHaveBeenCalledTimes(2);
  });

  it("should do nothing on flush when no trailing call is pending", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    throttled.flush();
    expect(fn).not.toHaveBeenCalled();
    throttled(1);
    throttled.flush();
    expect(fn).toHaveBeenCalledTimes(1);
  });

  it("should report pending state", () => {
    const fn = jest.fn();
    const throttled = throttle(fn, 100);

    expect(throttled.pending()).toBe(false);
    throttled();
    expect(throttled.pending()).toBe(false);
    throttled();
    expect(throttled.pending()).toBe(true);
    jest.advanceTimersByTime(100);
    expect(throttled.pending()).toBe(false);
    throttled();
    throttled.cancel();
    expect(throttled.pending()).toBe(false);
  });
});

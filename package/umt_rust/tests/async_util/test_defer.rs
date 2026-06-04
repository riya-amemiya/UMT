use std::thread;
use std::time::Duration;
use umt_rust::async_util::umt_defer;

#[test]
fn test_defer_resolves_with_the_given_value() {
    let d = umt_defer::<i32, String>();
    d.resolve(42);
    assert_eq!(d.wait(), Ok(42));
}

#[test]
fn test_defer_rejects_with_the_given_reason() {
    let d = umt_defer::<i32, String>();
    d.reject("deferred error".to_string());
    assert_eq!(d.wait(), Err("deferred error".to_string()));
}

#[test]
fn test_defer_settles_from_another_thread() {
    let d = umt_defer::<&'static str, String>();
    let settler = d.settler();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _ = settler.send(Ok("delayed"));
    });
    assert_eq!(d.wait(), Ok("delayed"));
}

#[test]
fn test_defer_first_settlement_wins() {
    let d = umt_defer::<i32, String>();
    d.resolve(1);
    d.resolve(2);
    assert_eq!(d.wait(), Ok(1));
}

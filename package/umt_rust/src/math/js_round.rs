/// Rounds like JavaScript `Math.round`: ties go toward positive infinity.
///
/// NaN and infinities are returned unchanged.
#[inline]
pub(crate) fn js_round(value: f64) -> f64 {
    let floored = value.floor();
    if value - floored >= 0.5 {
        floored + 1.0
    } else {
        floored
    }
}

pub mod profiler;

pub use profiler::{GasProfile};
use soroban_sdk::Env;

/// Measure gas/footprint for a closure
pub fn profile<F>(env: &Env, f: F) -> GasProfile
where
    F: FnOnce(),
{
    let start = GasProfile::new(env);
    f();
    let end = GasProfile::new(env);
    start.delta(&end)
}
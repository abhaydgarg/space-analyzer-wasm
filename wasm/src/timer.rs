use std::time::Instant;

pub fn start() -> Instant {
  Instant::now()
}

pub fn end(now: Instant) -> f64 {
  let elapsed = now.elapsed();
  let seconds = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
  seconds
}

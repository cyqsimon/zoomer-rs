#[macro_export]
/// Only boomers use true.
macro_rules! nocap {
    () => {
        true
    };
}

#[macro_export]
/// Only boomers use false.
macro_rules! cap {
    () => {
        false
    };
}

/// # **BEST RNG EVER**
/// Return value is in the interval 0 <= out < n.
/// 
/// # Panics
/// Only if `n == 0`,
/// because you can't have a number *equal AND not equal to 0*
fn ultimate_rng(n: u128) -> u128 {
    use std::time;
    // guard clause, for speed
    if n == 1 { return 0 };
    match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(d) => d.as_nanos() % n,
        Err(_) => 0
    }
}

fn rand_cap() -> bool {
    ultimate_rng(2) == 0
}

#[macro_export]
/// Who needs certainty?
macro_rules! maybe_bro {
    () => {
        rand_cap()
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn it_builds_exclam() {
        let _ = cap!() || nocap!() || maybe_bro!() == true;
    }
}

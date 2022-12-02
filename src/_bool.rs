#[macro_export]
/// Only boomers use `true`.
macro_rules! nocap {
    () => {
        true
    };
}

#[macro_export]
/// Only boomers use `false`.
macro_rules! cap {
    () => {
        false
    };
}

#[macro_export]
/// fake news!
macro_rules! fake {
    () => {
        false
    };
}

#[macro_export]
/// Who needs certainty?
macro_rules! maybe_bro {
    () => {
        match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        {
            Ok(d) => d.as_nanos() % 2,
            Err(_) => 0
        } == 1 // 1 is truthy, 0 is falsy
    };
}

#[macro_export]
/// yesn't
macro_rules! yesnt {
    () => {
        maybe_bro!()
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn it_builds_exclam() {
        let _ = cap!() || nocap!() ||
            fake!() || maybe_bro!() || yesnt!()
            == true;
    }
}

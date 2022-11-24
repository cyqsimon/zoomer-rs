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

#[macro_export]
/// Who needs certainty?
macro_rules! maybe_bro {
    () => {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("omegalul")
            .as_micros()
            % 2
            == 0
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn it_builds_exclam() {
        let _ = cap!() || nocap!() || maybe_bro!() == true;
    }
}

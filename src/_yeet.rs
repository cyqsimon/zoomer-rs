#[macro_export]
/// One does not simply yeet.
macro_rules! yeet {
    () => {
        None?
    };
    // for legal reasons, I do not actually recommend yeeting your dog
    ($your_dawg: expr) => {
        Err($your_dawg)?
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn it_builds_exclam() {
        let _fuck_all = just_yeet();
        let _dawg = fancy_yeet();
    }

    fn fancy_yeet() -> Result<(), &'static str> {
        yeet!("Scooby Doo");
        Ok(())
    }

    fn just_yeet() -> Option<()> {
        yeet!();
        Some(())
    }
}

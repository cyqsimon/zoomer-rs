#[macro_export]
/// Not sure if people stopped saying "YOLO",
///
/// Or everyone that said "YOLO" has died.
macro_rules! yolo {
    ($yo_mama: block) => {
        unsafe { $yo_mama }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn it_builds_exclam() {
        yolo!({
            let mut yo_mama = 69usize;
            let so_fat = &mut yo_mama as *mut usize;
            assert_eq!(yo_mama, *so_fat);
        })
    }
}

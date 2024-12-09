#![feature(iter_map_windows)]

mod day_1;
mod day_2;
mod day_9;

#[macro_export]
macro_rules! prob {
    ($name:ident($file:expr) => |$in:ident| -> $ty:ty $b:block) => {
        paste::paste! {
            #[test]
            fn [<test_ $name>]() {
                use std::io::Read;

                let mut $in = String::new();
                std::fs::File::open(format!("inputs/{}", $file)).unwrap()
                    .read_to_string(&mut $in).unwrap();

                let start = std::time::Instant::now();
                let result = $name($in.as_str());
                println!("Day {} = {result} ({:?})", stringify!($name), start.elapsed());
            }

            #[allow(unused)]
            fn $name($in: &str) -> $ty $b
        }
    };
}

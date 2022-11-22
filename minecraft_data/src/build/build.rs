#[allow(unused_macros)]
macro_rules! log {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

mod blocks;

fn main() {
    blocks::build_blocks();
    println!("cargo:rerun-if-changed=data");
}

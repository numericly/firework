#[allow(unused_macros)]
macro_rules! log {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

mod biomes;
mod blocks;

fn main() {
    blocks::build_blocks();
    biomes::build_biomes();
    println!("cargo:rerun-if-changed=data");
}

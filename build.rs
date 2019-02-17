extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/common_dht_read.c")
        .file("src/pi_2_mmio.c")
        .file("src/pi_2_dht_read.c")
        .include("src")
        .flag("--sysroot=/home/rschwalk/raspi/sysroot")
        .flag("-std=gnu99")
        .compile("pi_2_dht");
}

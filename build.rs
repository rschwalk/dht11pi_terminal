extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/common_dht_read.c")
        .file("src/pi_2_mmio.c")
        .file("src/pi_2_dht_read.c")
        .target("arm-linux-gnu")
        .compiler("arm-linux-gnu-gcc")
        .compile("pi_2_dht");
}

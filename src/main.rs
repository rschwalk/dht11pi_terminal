mod pi_2_dht;

fn main() {
    let res = pi_2_dht::pi_2_dht_read(4);

    println!("Result: {:?}", res);
}


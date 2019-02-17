use cty;

extern "C" {
    fn pi_2_dht_read(
        typ: cty::c_int,
        pin: cty::c_int,
        humidity: *mut cty::c_float,
        temperature: *mut cty::c_float
        ) -> cty::c_int;
}

fn main() {
    let mut temp = 0.0;
    let mut hum = 0.0;

    let _val = unsafe {
        pi_2_dht_read(11, 4, &mut hum, &mut temp);
    };

    println!("Temperature: {}C°, Humidity: {}%", temp, hum);

}


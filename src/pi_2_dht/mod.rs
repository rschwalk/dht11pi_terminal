mod pi_2_mmio;

// This is the only processor specific magic value, the maximum amount of time to
// spin in a loop before bailing out and considering the read a timeout.  This should
// be a high value, but if you're running on a much faster platform than a Raspberry
// Pi or Beaglebone Black then it might need to be increased.
const DHT_MAXCOUNT: i32 = 32000;

// Number of bit pulses to expect from the DHT.  Note that this is 41 because
// the first pulse is a constant 50 microsecond pulse, with 40 pulses to represent
// the data afterwards.
const DHT_PULSES: i32 = 41;

pub fn pi_2_dht_read(pin: i32) -> (i32, i32) {
    let mut temperature = 0;
    let mut humidity = 0;

    (temperature, humidity)
}

use rand::Rng;

pub fn gen_ran() -> u8 {
    // making rand generator
    let mut rng = rand::thread_rng();
    let n: u8 = rng.gen();
    n
}

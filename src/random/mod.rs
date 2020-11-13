use rand::Rng;

pub fn roll(max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1, max)
}

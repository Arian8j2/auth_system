use rand::Rng;

pub fn generate_random_six_digit_code() -> u32 {
    let mut rng = rand::thread_rng();
    (0..=6)
        .map(|_| rng.gen_range(1..10).to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u32>()
        .unwrap()
}

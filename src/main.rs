use std::{
    io::stdin,
    time::Instant
};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    println!("How many battles would you like to attempt?");
    let mut buf: String = "".to_owned();
    stdin().read_line(&mut buf).unwrap();

    let battles = u32::from_str_radix(&buf.trim(), 10).unwrap();
    let mut played_battles: u32 = 0;

    let mut most_paralyzed_turns: u8 = 0;

    let start_time = Instant::now();
    while played_battles < battles && most_paralyzed_turns < 177 {
        let mut paralyzed: u8 = 0;
        for _ in 0..231 {
            if rng.gen_range(0..4 as u8) == 0 {
                paralyzed += 1;
            }
        }
        if paralyzed > most_paralyzed_turns {
            most_paralyzed_turns = paralyzed;
        }
        played_battles += 1;
    }
    let duration = start_time.elapsed();

    println!("Highest amount of paralyzed turns: {most_paralyzed_turns}");
    println!("Number of battles simulated: {played_battles}");
    println!("The code took: {}s", duration.as_secs_f64());
}

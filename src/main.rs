use std::{
    io::stdin, thread::{self, JoinHandle}, time::{Duration, Instant}
};
use rand::{thread_rng, Rng};

fn main() {
    fn run_simulations(battles: u32) -> (u8, u32) {
        let mut rng = thread_rng();
        let mut most_paralyzed: u8 = 0;
        let mut battles_played: u32 = 0;
        for i in 1..battles + 1 {
            battles_played = i;
            let mut paralyzed_turns: u8 = 0;
            for _ in 0..231 {
                if rng.gen_range(0..4 as u8) == 0 {
                    paralyzed_turns += 1;
                }
            }
            if paralyzed_turns > most_paralyzed {
                most_paralyzed = paralyzed_turns;
            }
            if most_paralyzed >= 177 {
                return (most_paralyzed, battles_played);
            }
        }
        return (most_paralyzed, battles_played);
    }

    println!("How many battles would you like to attempt?");
    let mut buf: String = "".to_owned();
    stdin().read_line(&mut buf).unwrap();

    let mut battles = u32::from_str_radix(&buf.trim(), 10).unwrap();
    
    let thread_count = thread::available_parallelism().unwrap();
    let mut threads: Vec<JoinHandle<(u8, u32)>> = vec![];

    println!("Running with {thread_count} threads");

    let start_time: Instant = Instant::now();

    for i in 0..thread_count.into() {
        let battles_to_run = battles / (thread_count.get() - i) as u32;
        let thread: JoinHandle<(u8, u32)> = thread::spawn(move || run_simulations(battles_to_run));
        threads.push(thread);
        battles -= battles_to_run;
    }

    let mut most_paralyzed_turns = 0;
    let mut played_battles = 0;
    for thread in threads {
        let (most_paralyzed_thread, played_battles_thread) = thread.join().unwrap();
        played_battles += played_battles_thread;
        if most_paralyzed_thread > most_paralyzed_turns {
            most_paralyzed_turns = most_paralyzed_thread
        }
    }

    let duration: Duration = start_time.elapsed();

    println!("Highest amount of paralyzed turns: {}", most_paralyzed_turns);
    println!("Number of battles simulated: {}", played_battles);
    println!("The code took: {}s", duration.as_secs_f64());
}

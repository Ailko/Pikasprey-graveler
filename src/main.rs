use std::{
    io::stdin, thread::{self, JoinHandle}, time::{Duration, Instant}
};
use rand::{thread_rng, Rng};

fn main() {
    // Distribution of all possible outcomes for 21 turns of 4 options
    fn rng_to_paralyzed_turns(rng: u64) -> u8 {
        if rng < 10460353203 {return 0}
        else if rng < 83682825624 {return 1}
        else if rng < 327757733694 {return 2}
        else if rng < 843026984064 {return 3}
        else if rng < 1615930859619 {return 4}
        else if rng < 2491888585248 {return 5}
        else if rng < 3270517674696 {return 6}
        else if rng < 3826681310016 {return 7}
        else if rng < 4151110097286 {return 8}
        else if rng < 4307316550416 {return 9}
        else if rng < 4369799131668 {return 10}
        else if rng < 4390626658752 {return 11}
        else if rng < 4396412082942 {return 12}
        else if rng < 4397747180832 {return 13}
        else if rng < 4398001485192 {return 14}
        else if rng < 4398041043648 {return 15}
        else if rng < 4398045988455 {return 16}
        else if rng < 4398046473240 {return 17}
        else if rng < 4398046509150 {return 18}
        else if rng < 4398046511040 {return 19}
        else if rng < 4398046511103 {return 20}
        else {return 21}

    }

    fn run_simulations(battles: u32) -> (u8, u32) {
        let mut rng = thread_rng();
        let mut most_paralyzed: u8 = 0;
        let mut battles_played: u32 = 0;
        for i in 1..battles + 1 {
            battles_played = i;
            let mut paralyzed_turns: u8 = 0;
            for _ in 0..11 {
                paralyzed_turns += rng_to_paralyzed_turns(rng.gen_range(0..4398046511104 as u64));
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

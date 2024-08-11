use std::{
    io::stdin, sync::{Arc, Mutex, MutexGuard}, thread::{self, JoinHandle}, time::{Duration, Instant}
};
use rand::{thread_rng, Rng};

fn main() {
    fn increment_played_battles(mut played_battles: MutexGuard<u32>, battles: u32) -> Option<()> {
        if *played_battles >= battles {
            return None;
        }
        *played_battles += 1;
        return Some(())
    }

    fn update_most_paralyzed_turns(mut most_paralyzed_turns: MutexGuard<u8>, update_value: u8) {
        if *most_paralyzed_turns >= update_value {
            return;
        }
        *most_paralyzed_turns = update_value.clone();
    }

    println!("How many battles would you like to attempt?");
    let mut buf: String = "".to_owned();
    stdin().read_line(&mut buf).unwrap();

    let battles = u32::from_str_radix(&buf.trim(), 10).unwrap();
    let played_battles = Arc::new(Mutex::new(0));

    let most_paralyzed_turns = Arc::new(Mutex::new(0));
    
    let thread_count = thread::available_parallelism().unwrap();
    let mut threads: Vec<JoinHandle<()>> = vec![];

    println!("Running with {thread_count} threads");

    let start_time: Instant = Instant::now();

    for _ in 0..thread_count.into() {
        let played_battles = Arc::clone(&played_battles);
        let most_paralyzed_turns = Arc::clone(&most_paralyzed_turns);

        let thread: JoinHandle<()> = thread::spawn(move || {
            let mut rng = thread_rng();
            while increment_played_battles(played_battles.lock().unwrap(), battles).is_some() && {*most_paralyzed_turns.lock().unwrap() < 177} {
                let mut paralyzed_turns: u8 = 0;
                for _ in 0..231 {
                    if rng.gen_range(0..4 as u8) == 0 {
                        paralyzed_turns += 1;
                    }
                }
                update_most_paralyzed_turns(most_paralyzed_turns.lock().unwrap(), paralyzed_turns)
            }
        });

        threads.push(thread);
    }

    for thread in threads {
        thread.join().unwrap();
    }

    let duration: Duration = start_time.elapsed();

    println!("Highest amount of paralyzed turns: {}", *most_paralyzed_turns.lock().unwrap());
    println!("Number of battles simulated: {}", *played_battles.lock().unwrap());
    println!("The code took: {}s", duration.as_secs_f64());
}

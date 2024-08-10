use std::{
    io::stdin, sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::{Duration, Instant}
};
use rand::{thread_rng, Rng};

fn main() {
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
            let mut battle: u32;
            {
                let mut played_battles_local = played_battles.lock().unwrap();
                battle = played_battles_local.clone();
                if battle >= battles {
                    return;
                }
                *played_battles_local += 1;
            }
            while battle < battles && {*most_paralyzed_turns.lock().unwrap() < 177} {
                let mut paralyzed_turns: u8 = 0;
                for _ in 0..231 {
                    if rng.gen_range(0..4 as u8) == 0 {
                        paralyzed_turns += 1;
                    }
                }
                let mut most_paralyzed_turns_local = most_paralyzed_turns.lock().unwrap();
                if paralyzed_turns > *most_paralyzed_turns_local {
                    *most_paralyzed_turns_local = paralyzed_turns;
                }  
                {
                    let mut played_battles_local = played_battles.lock().unwrap();
                    battle = played_battles_local.clone();
                    if battle >= battles {
                        return;
                    }
                    *played_battles_local += 1;
                }
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

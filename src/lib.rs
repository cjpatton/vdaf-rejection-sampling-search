use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::spawn,
};

use prio::{
    codec::Decode,
    field::FieldElementWithInteger,
    vdaf::prg::{Prg, PrgSha3, Seed, SeedStream},
};
use rand::{thread_rng, Rng};

pub struct Config {
    jobs: usize,
    prg_iterations: usize,
}

impl Config {
    pub fn new(jobs: usize, prg_iterations: usize) -> Config {
        Config {
            jobs,
            prg_iterations,
        }
    }
}

pub fn search<F>(config: Config)
where
    F: FieldElementWithInteger,
    F::Integer: Into<u128>,
{
    let done = Arc::new(AtomicBool::new(false));
    let mut join_handles = Vec::with_capacity(config.jobs);
    for _ in 0..config.jobs {
        join_handles.push({
            let done = Arc::clone(&done);
            spawn(move || {
                let custom = b"";
                let binder = b"";

                let mut rng = thread_rng();
                let mut buffer = [0u8; 16];
                let modulus: u128 = F::modulus().into();

                while !done.load(Ordering::Relaxed) {
                    let seed_bytes = rng.gen::<[u8; 16]>();
                    let seed = Seed::get_decoded(&seed_bytes).unwrap();
                    let mut seed_stream = PrgSha3::seed_stream(&seed, custom, binder);
                    for i in 0..config.prg_iterations {
                        seed_stream.fill(&mut buffer[0..F::ENCODED_SIZE]);
                        let candidate = u128::from_le_bytes(buffer);
                        if candidate >= modulus {
                            seed_stream.fill(&mut buffer[0..F::ENCODED_SIZE]);
                            let next = u128::from_le_bytes(buffer);
                            println!(
                                "Rejection found, seed = {seed:02x?}, offset = {i} (length {})",
                                i + 1
                            );
                            println!("rejected = {candidate}, next = {next}");
                            done.store(true, Ordering::Relaxed);
                            break;
                        }
                    }
                }
            })
        });
    }
    for join_handle in join_handles {
        join_handle.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use prio::field::Field96;

    use crate::search;

    #[test]
    fn field96() {
        // This search should complete quickly.
        search::<Field96>(crate::Config::new(2, 1000));
    }
}

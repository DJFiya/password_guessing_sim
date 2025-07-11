//Password generator / solver for password guessing simulation

pub use crate::config::Config;

pub struct Generator {
    pub current: Vec<usize>,
    pub config: Config,
}


impl Generator {
    // Creates a new password guess generator.
    pub fn new(config: Config) -> Self {
        Generator {
            current: vec![0],
            config,
        }
    }


    pub fn next_guess(&mut self) -> Option<String> {
        // Generate the next password guess based on the current state
        while self.current.len() <= self.config.max_length {
            let guess: String = self.current
                .iter()
                .map(|&i| self.config.charset[i])
                .collect();

            self.increment();

            if guess.len() >= self.config.min_length {
                return Some(guess);
            }
        }
        None
    }


    fn increment(&mut self) {
        // Increment the current guess index
        let base = self.config.charset.len();
        let mut index = self.current.len() - 1;

        loop {
            self.current[index] += 1;

            if self.current[index] < base {
                break;
            } else {
                self.current[index] = 0;
                if index == 0 {
                    if self.current.len() >= self.config.max_length {
                        break;
                    }
                    self.current.insert(0, 0);
                    break;
                } else {
                    index -= 1;
                }
            }
        }
    }
}

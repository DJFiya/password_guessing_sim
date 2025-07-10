use crate::utils::get_charset;

pub struct Generator {
    current: Vec<usize>,
    charset: Vec<char>,
}

impl Generator {
    // Creates a new password guess generator.
    pub fn new() -> Self {
        Generator {
            current: vec![0],
            charset: get_charset(),
        }
    }

    pub fn next_guess(&mut self) -> String {
        // Convert indices to characters
        let guess: String = self.current
            .iter()
            .map(|&i| self.charset[i])
            .collect();

        self.increment();

        guess
    }

    fn increment(&mut self) {
        // Increment the current guess
        let base = self.charset.len();
        let mut index = self.current.len() - 1;

        loop {
            self.current[index] += 1;

            if self.current[index] < base {
                break;
            } else {
                self.current[index] = 0;
                if index == 0 {
                    self.current.insert(0, 0);
                    break;
                } else {
                    index -= 1;
                }
            }
        }
    }
}

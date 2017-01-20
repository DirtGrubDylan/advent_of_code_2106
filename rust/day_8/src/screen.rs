use std::fmt;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Screen {
    pub display: VecDeque<VecDeque<char>>,
}


impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.display {
            write!(f, "\n{}", line.iter().cloned().collect::<String>())?;
        }

        Ok(())
    }
}
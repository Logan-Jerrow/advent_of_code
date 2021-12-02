#[derive(Debug, Default)]
pub struct Submarine {
    depth: u32,
    horizontal: u32,
}

impl Submarine {
    pub fn move_sub(&mut self, command: Commands) {
        match command {
            Commands::Forward(x) => self.horizontal += x,
            Commands::Down(x) => self.depth += x,
            Commands::Up(x) => self.depth -= x,
        };
    }

    pub fn cross(&self) -> u32 {
        self.depth * self.horizontal
    }
}
#[derive(Debug)]
pub enum Commands {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: [Commands; 6] = [
        Commands::Forward(5),
        Commands::Down(5),
        Commands::Forward(8),
        Commands::Up(3),
        Commands::Down(8),
        Commands::Forward(2),
    ];

    #[test]
    fn sub() {
        let mut sub = Submarine::default();
        INPUT.into_iter().for_each(|c| sub.move_sub(c));
        assert_eq!(sub.cross(), 150);
    }
}

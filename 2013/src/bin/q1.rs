use std::io;

struct Clock {
    minutes_fast: usize,
    hours: usize,
    minutes: usize,
}

impl Clock {
    const HOURS_IN_DAY: usize = 24;
    const MINUTES_IN_HOUR: usize = 60;

    fn new(minutes_fast: usize) -> Self {
        Self { minutes_fast, hours: 0, minutes: 0 }
    }

    fn increment(&mut self) {
        self.hours += 1;
        self.minutes += self.minutes_fast;

        if self.minutes >= Self::MINUTES_IN_HOUR {
            self.hours += self.minutes / Self::MINUTES_IN_HOUR;
            self.minutes %= Self::MINUTES_IN_HOUR;
        }

        if self.hours >= Self::HOURS_IN_DAY {
            self.hours %= Self::HOURS_IN_DAY;
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    let clock1_minutes_fast: usize = buf.split_whitespace().nth(0).unwrap().parse().unwrap();
    let clock2_minutes_fast: usize = buf.split_whitespace().nth(1).unwrap().parse().unwrap();

    let mut clock1 = Clock::new(clock1_minutes_fast);
    let mut clock2 = Clock::new(clock2_minutes_fast);

    loop {
        clock1.increment();
        clock2.increment();

        if clock1 == clock2 {
            break;
        }
    }

    println!("{:0>2}:{:0>2}", clock1.hours, clock1.minutes);

    Ok(())
}


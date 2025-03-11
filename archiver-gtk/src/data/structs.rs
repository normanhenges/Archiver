use {
    std::fmt,
    once_cell::sync::Lazy,
    regex::Regex,
};

#[derive(Debug)]
pub struct Day {
    year: u16,
    month: u8,
    day: u8,
}

impl Day {
    pub fn new(year: u16, month: u8, day: u8) -> Day {
        Day { day, month, year }
    }

    const MONTH_NAMES: [&'static str; 12] = [
        "Januar", "Februar", "März", "April", "Mai", "Juni",
        "Juli", "August", "September", "Oktober", "November", "Dezember"
    ];

    pub fn month_name(&self) -> &'static str {
        Self::MONTH_NAMES[(self.month - 1) as usize]
    }

    // Create a new Day from a string in the format YYYY-MM-DD
    pub fn from_string(text: &str) -> Result<Day, String> {
        if check_day_regex(text) {
            let year = text[0..4].parse().map_err(|_| "Invalid year")?;
            let month = text[5..7].parse().map_err(|_| "Invalid month")?;
            let day = text[8..10].parse().map_err(|_| "Invalid day")?;
            Ok(Day::new(year, month, day))
        } else {
            Err("Invalid day format: {}".to_string())
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}. {:02} {:04}", self.day, self.month_name(), self.year)
    }
}

// Statically compile regex using once_cell to increase performance
fn check_day_regex(text: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());
    RE.is_match(text)
}
use time::Date;
use time::Month;
use time::Weekday;

fn main() {
    let start = Date::from_calendar_date(1901, Month::January, 1).unwrap();
    let result = DateIterator { date: start }
        .take_while(|d| d.year() < 2001)
        .filter(|d| d.day() == 1 && d.weekday() == Weekday::Sunday)
        .count();
    println!("Problem 19 solution: {}", result);
}

struct DateIterator {
    date: Date,
}

impl Iterator for DateIterator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        match self.date.next_day() {
            Some(next_date) => {
                self.date = next_date;
                Some(next_date)
            }
            None => None
        }
    }
}

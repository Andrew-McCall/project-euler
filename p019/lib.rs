/* p19: Counting Sundays */
pub fn solution(start: Date, end: Date) -> u64 {
    let mut sundays = 0;
    let mut current = start;

    while current != end {
        if current.year >= end.year
            && current.month as u8 > end.month as u8
            && current.day > end.day
        {
            panic!("end date was wrong");
        }

        current.day += 1;
        current.weekday = current.weekday.incr();
        if current.day
            > current
                .month
                .days_in_month(is_leap_year(current.year as u64))
        {
            current.day = 1;
            current.month = current.month.incr();
            if current.day == 1 && current.month == Month::January {
                current.year += 1;
            }
        }

        if current.day == 1 && current.weekday == DayOfWeek::Sunday {
            sundays += 1;
        }
    }

    sundays
}

#[test]
fn test() {
    assert_eq!(
        solution(
            Date {
                year: 1901,
                month: Month::January,
                day: 1,
                weekday: DayOfWeek::Tuesday
            },
            Date {
                year: 2000,
                month: Month::December,
                day: 31,
                weekday: DayOfWeek::Sunday
            }
        ),
        0
    );
}

fn is_leap_year(year: u64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: u32,
    pub month: Month,
    pub day: u8,
    pub weekday: DayOfWeek,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    pub fn incr(&self) -> DayOfWeek {
        let next = (*self as u8 + 1) % 7;
        unsafe { std::mem::transmute(next) }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn days_in_month(&self, is_leap_year: bool) -> u8 {
        match self {
            Month::January => 31,
            Month::February => {
                if is_leap_year {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub fn incr(&self) -> Month {
        let next = (*self as u8 + 1) % 12;
        unsafe { std::mem::transmute(next) }
    }
}

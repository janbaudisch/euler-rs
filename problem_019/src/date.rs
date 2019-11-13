pub struct Date {
    pub weekday: Weekday,
    pub day: u8,
    pub month: Month,
    pub year: u16,
}

impl Date {
    pub fn next(self) -> Self {
        let weekday = self.weekday.next();
        let mut day = self.day + 1;
        let mut month = self.month;

        if day > 28 && self.month == Month::February {
            if (self.year % 4 == 0 && self.year % 100 != 0) || self.year % 400 == 0 {
                if day == 30 {
                    month = month.next();
                    day = 1;
                }
            } else {
                month = month.next();
                day = 1;
            }
        } else if (day == 31 && self.month.is_short()) || (day == 32 && !self.month.is_short()) {
            month = month.next();
            day = 1;
        }

        let year = if month == Month::January && day == 1 {
            self.year + 1
        } else {
            self.year
        };

        Self {
            weekday,
            day,
            month,
            year,
        }
    }
}

#[derive(PartialEq)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    pub fn next(self) -> Weekday {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    Juli,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn next(self) -> Month {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::Juli,
            Month::Juli => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }

    pub fn is_short(self) -> bool {
        self == Month::April
            || self == Month::June
            || self == Month::September
            || self == Month::November
    }
}

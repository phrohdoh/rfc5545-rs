use std::fmt;

pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl fmt::Display for Frequency {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Frequency::Secondly => "SECONDLY",
            Frequency::Minutely => "MINUTELY",
            Frequency::Hourly => "HOURLY",
            Frequency::Daily => "DAILY",
            Frequency::Weekly => "WEEKLY",
            Frequency::Monthly => "MONTHLY",
            Frequency::Yearly => "YEARLY",
        }.to_string())
    }
}

pub enum RecurRulePart {
    Freq(Frequency),
}

pub struct RecurrenceRule {
    pub recur_rule_parts: Vec<RecurRulePart>,
}

impl fmt::Display for RecurrenceRule {
    fn  fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RRULE:")?;

        for part in self.recur_rule_parts.iter() {
            match part {
                RecurRulePart::Freq(freq) => write!(f, "FREQ={}", freq)?,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Frequency, RecurRulePart, RecurrenceRule};

    #[test]
    fn rr_with_freq_monthly_display() {
        let rr = RecurrenceRule {
            recur_rule_parts: vec![RecurRulePart::Freq(Frequency::Monthly)],
        };

        assert_eq!(format!("{}", rr), "RRULE:FREQ=MONTHLY");
    }
}

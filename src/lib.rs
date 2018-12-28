use std::fmt;

#[derive(PartialEq, Clone, Debug)]
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
    Count(u64),
    Interval(u64),
}

pub struct RecurrenceRule {
    pub recur_rule_parts: Vec<RecurRulePart>,
}

impl RecurrenceRule {
    pub fn new_single() -> Self {
        Self { recur_rule_parts: vec![] }
    }

    pub fn new_with_parts(recur_rule_parts: Vec<RecurRulePart>) -> Self {
        Self { recur_rule_parts }
    }
}

impl fmt::Display for RecurrenceRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RRULE:")?;

        for part in self.recur_rule_parts.iter() {
            match part {
                RecurRulePart::Freq(freq) => write!(f, "FREQ={}", freq)?,
                RecurRulePart::Count(count) => write!(f, "COUNT={}", count)?,
                RecurRulePart::Interval(interval) => write!(f, "INTERVAL={}", interval)?,
            };

            write!(f, ";")?;
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

        assert_eq!(format!("{}", rr), "RRULE:FREQ=MONTHLY;");
    }

    #[test]
    fn rr_with_count_5_display() {
        let rr = RecurrenceRule {
            recur_rule_parts: vec![RecurRulePart::Count(5)],
        };

        assert_eq!(format!("{}", rr), "RRULE:COUNT=5;");
    }

    #[test]
    fn rr_with_interval_5_display() {
        let rr = RecurrenceRule::new_with_parts(vec![
            RecurRulePart::Interval(5),
        ]);

        let sut = rr.to_string();
        assert_eq!(sut, "RRULE:INTERVAL=5;");
    }

    #[test]
    fn rr_multiple_parts() {
        // Every 2 months for a total of 9 occurences (over 2 * 9 months)
        let rr = RecurrenceRule::new_with_parts(vec![
            RecurRulePart::Freq(Frequency::Monthly),
            RecurRulePart::Count(9),
            RecurRulePart::Interval(2),
        ]);

        // TODO: Test for `;` between each part
        // TODO: Test that parts do not contain whitespace (surrounding only?)
        let sut = format!("{}", rr);
        assert!(sut.starts_with("RRULE:"));
        assert!(sut.contains("INTERVAL=2"));
        assert!(sut.contains("FREQ=MONTHLY"));
        assert!(sut.contains("COUNT=9"));
    }
}

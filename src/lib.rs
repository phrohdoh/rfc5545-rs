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
    Count(u64),
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
                RecurRulePart::Count(count) => write!(f, "COUNT={}", count)?,
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
    fn rr_with_freq_monthly_count_5_display() {
        let rr = RecurrenceRule {
            recur_rule_parts: vec![
                RecurRulePart::Freq(Frequency::Monthly),
                RecurRulePart::Count(5)
            ],
        };

        // TODO: Test for `;` between each part
        // TODO: Test that parts do not contain whitespace (surrounding only?)
        let sut = format!("{}", rr);
        assert!(sut.starts_with("RRULE:"));
        assert!(sut.contains("FREQ=MONTHLY"));
        assert!(sut.contains("COUNT=5"));
    }
}

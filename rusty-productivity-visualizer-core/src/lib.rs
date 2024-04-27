use chrono::{NaiveDate, TimeDelta};

pub mod heatmap;

pub trait DataVisualizer {
    type Visualized;
    fn add_activity(&mut self, activity: Activity);
    fn visualize(&self) -> Self::Visualized;
}

pub struct Activity {
    name: String,
    date: NaiveDate,
    duration: TimeDelta,
}

impl Activity {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn duration(&self) -> TimeDelta {
        self.duration
    }
}

pub struct ActivityBuilder;

impl ActivityBuilder {
    pub fn name(self, name: String) -> DateActivityBuilder {
        DateActivityBuilder { name }
    }
}

struct DateActivityBuilder {
    name: String,
}

impl DateActivityBuilder {
    pub fn date(self, date: NaiveDate) -> DurationActivityBuilder {
        DurationActivityBuilder {
            name: self.name,
            date,
        }
    }
}

struct DurationActivityBuilder {
    name: String,
    date: NaiveDate,
}

impl DurationActivityBuilder {
    pub fn duration(self, duration: TimeDelta) -> Activity {
        Activity {
            name: self.name,
            date: self.date,
            duration,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn activity_accessors(name in ".*", day in -365i64*100..365*100, duration in -10000i64..10000) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(duration);
            let activity = Activity {
                name: name.clone(),
                date,
                duration,
            };
            assert_eq!(name, activity.name());
            assert_eq!(date, activity.date());
            assert_eq!(duration, activity.duration());
        }
        #[test]
        fn activity_builder_accessors(name in ".*", day in -365i64*100..365*100, duration in -10000i64..10000) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(duration);
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            assert_eq!(name, activity.name());
            assert_eq!(date, activity.date());
            assert_eq!(duration, activity.duration());
        }
    }
}

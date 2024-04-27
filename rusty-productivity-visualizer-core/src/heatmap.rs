use std::collections::HashMap;

use chrono::{NaiveDate, TimeDelta};

use crate::{Activity, DataVisualizer};

#[derive(Default)]
pub struct Heatmap {
    activities: Vec<Activity>,
}

impl DataVisualizer for Heatmap {
    type Visualized = HeatmapVisualized;
    fn add_activity(&mut self, activity: Activity) {
        self.activities.push(activity);
    }
    fn visualize(&self) -> Self::Visualized {
        let mut dates: Vec<_> = self.activities
            .iter()
            .map(|date| {
                let duration = date.duration();
                let intensity = (duration.num_seconds() as f32 / 86400.0).clamp(0.0, 1.0);
                HeatmapDay {
                    date: date.date,
                    intensity,
                }
            })
            .collect();
        dates.sort_by(|a, b| a.date.cmp(&b.date));
        HeatmapVisualized {
            dates,
        }
    }
}

pub struct HeatmapVisualized {
    dates: Vec<HeatmapDay>,
}

impl HeatmapVisualized {
    pub fn dates(&self) -> &[HeatmapDay] {
        &self.dates
    }
}

pub struct HeatmapDay {
    date: NaiveDate,
    intensity: f32,
}

impl HeatmapDay {
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::ActivityBuilder;

    use super::*;
    use chrono::Duration;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn heatmap_add_activity(name in ".*", day in -365i64*100..365*100, duration in -10000i64..10000) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(duration);
            let mut heatmap = Heatmap::default();
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            heatmap.add_activity(activity);
            assert_eq!(1, heatmap.activities.len());
            let activity = &heatmap.activities[0];
            assert_eq!(name, activity.name());
            assert_eq!(date, activity.date());
            assert_eq!(duration, activity.duration());
        }
        #[test]
        fn heatmap_visualize(name in ".*", day in -365i64*100..365*100, duration in -10000i64..10000) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(duration);
            let mut heatmap = Heatmap::default();
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            heatmap.add_activity(activity);
            let visualized = heatmap.visualize();
            let date = visualized.dates();
            assert_eq!(1, date.len());
            let date = &date[0];
            let seconds = duration.num_seconds();
            if seconds < 0 {
                assert_eq!(0.0, date.intensity());
            } else if seconds > 86400 {
                assert_eq!(1.0, date.intensity());
            } else {
                assert_eq!(seconds as f32 / 86400.0, date.intensity());
            }
        }
        #[test]
        fn heatmap_visualize_between_day_duration(name in ".*", day in -365i64*100..365*100, duration in 0i64..=86400) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(duration);
            let mut heatmap = Heatmap::default();
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            heatmap.add_activity(activity);
            let visualized = heatmap.visualize();
            let date = visualized.dates();
            assert_eq!(1, date.len());
            let date = &date[0];
            let seconds = duration.num_seconds();
            assert_eq!(seconds as f32 / 86400.0, date.intensity());
        }
        #[test]
        fn heatmap_visualize_negative_duration(name in ".*", day in -365i64*100..365*100) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(-100);
            let mut heatmap = Heatmap::default();
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            heatmap.add_activity(activity);
            let visualized = heatmap.visualize();
            let date = visualized.dates();
            assert_eq!(1, date.len());
            let date = &date[0];
            let seconds = duration.num_seconds();
            assert_eq!(0.0, date.intensity());
        }
        #[test]
        fn heatmap_visualize_above_day_duration(name in ".*", day in -365i64*100..365*100) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let duration = TimeDelta::seconds(90000);
            let mut heatmap = Heatmap::default();
            let activity = ActivityBuilder.name(name.clone())
                .date(date)
                .duration(duration);
            heatmap.add_activity(activity);
            let visualized = heatmap.visualize();
            let date = visualized.dates();
            assert_eq!(1, date.len());
            let date = &date[0];
            let seconds = duration.num_seconds();
            assert_eq!(1.0, date.intensity());
        }
        #[test]
        fn heatmap_visualized_dates_accessor(day in -365i64*100..365*100, intensity in -100f32..100.0) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let day = HeatmapDay {
                date,
                intensity,
            };
            let visualized = HeatmapVisualized {
                dates: vec![day],
            };
            let dates = visualized.dates();
            assert_eq!(1, dates.len());
            assert_eq!(dates[0].date(), visualized.dates[0].date());
            assert_eq!(dates[0].intensity(), visualized.dates[0].intensity());
        }
        #[test]
        fn heatmap_day_accessors(day in -365i64*100..365*100, intensity in -100f32..100.0) {
            let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap() - Duration::days(day);
            let day = HeatmapDay {
                date,
                intensity,
            };
            assert_eq!(date, day.date());
            assert_eq!(intensity, day.intensity());
        }
    }
}

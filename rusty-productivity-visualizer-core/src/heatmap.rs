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
        todo!()
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
    intensity: f32,
}

impl HeatmapDay {
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
            //let visualized = heatmap.visualize();
        }
        #[test]
        fn heatmap_visualized_dates_accessor(intensity in -100f32..100.0) {
            let day = HeatmapDay { intensity };
            let visualized = HeatmapVisualized {
                dates: vec![day],
            };
            let dates = visualized.dates();
            assert_eq!(1, dates.len());
            assert_eq!(dates[0].intensity(), visualized.dates[0].intensity());
        }
        #[test]
        fn heatmap_day_intensity_accessor(intensity in -100f32..100.0) {
            let day = HeatmapDay {
                intensity,
            };
            assert_eq!(intensity, day.intensity());
        }
    }
}

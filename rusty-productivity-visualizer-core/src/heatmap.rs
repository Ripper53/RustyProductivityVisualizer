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
    use proptest::prelude::*;

    #[test]
    fn heatmap_add_activity() {
        let mut heatmap = Heatmap::default();
        const NAME: &str = "TEST_NAME";
        const DATE: NaiveDate = NaiveDate::MIN;
        const DURATION: TimeDelta = TimeDelta::zero();
        let activity = ActivityBuilder.name(NAME.into())
            .date(DATE)
            .duration(DURATION);
        heatmap.add_activity(activity);
        assert_eq!(1, heatmap.activities.len());
        let activity = &heatmap.activities[0];
        assert_eq!(NAME, activity.name());
        assert_eq!(DATE, activity.date());
        assert_eq!(DURATION, activity.duration());
    }
    #[test]
    fn heatmap_visualize() {
        let mut heatmap = Heatmap::default();
        const NAME: &str = "TEST_NAME";
        const DATE: NaiveDate = NaiveDate::MIN;
        const DURATION: TimeDelta = TimeDelta::zero();
        let activity = ActivityBuilder.name(NAME.into())
            .date(DATE)
            .duration(DURATION);
        heatmap.add_activity(activity);
        //let visualized = heatmap.visualize();
    }
    proptest! {
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

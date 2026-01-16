use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::UnboundedReceiver;
use tracing::info;

use crate::{
    config::{ThresholdMin, Thresholds},
    notifier::NotificationDispatcher,
    stats::{Report, StatisticsAccumulator},
};

pub async fn start_worker(
    mut rx: UnboundedReceiver<Vec<Report>>,
    dispatcher: Arc<NotificationDispatcher>,
    batch_interval_secs: u64,
    thresholds: Thresholds,
) {
    let mut accumulator = StatisticsAccumulator::default();
    let mut tick = tokio::time::interval(Duration::from_secs(batch_interval_secs));

    fn meets_min_threshold(count: usize, thresholds: &Thresholds) -> bool {
        match thresholds.threshold_min {
            ThresholdMin::Green => true,
            ThresholdMin::Orange => count >= thresholds.orange,
            ThresholdMin::Red => count >= thresholds.red,
        }
    }

    loop {
        tokio::select! {
            Some(reports) = rx.recv() => {
                accumulator.add_reports(&reports);
            }
            _ = tick.tick() => {
                if accumulator.total_count > 0 && meets_min_threshold(accumulator.total_count, &thresholds) {
                    let stats = accumulator.to_statistics();
                    info!("Dispatching statistics: total={} distinct={}", stats.total_count, stats.distinct_cidrs);
                    dispatcher.dispatch(stats).await;
                }
                accumulator.reset();
            }
        }
    }
}

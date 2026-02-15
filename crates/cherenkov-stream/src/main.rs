use timely::dataflow::operators::{Map, Filter, Inspect};
use tracing::{info, warn};

mod anomaly;
mod window;
mod correlation;

use anomaly::{AnomalyDetector, Severity};
use window::TumblingWindow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("Starting Cherenkov Stream Processor");
    
    let mut detector = AnomalyDetector::new();
    
    timely::execute_from_args(std::env::args(), move |worker| {
        let index = worker.index();
        
        worker.dataflow::<u64, _, _>(|scope| {
            let stream = scope.input_from(&mut vec![].into_iter());
            
            stream
                .window(TumblingWindow::new(60))
                .map(|window| {
                    detector.detect(window)
                })
                .filter(|anomaly| anomaly.is_some())
                .inspect(|anomaly| {
                    info!("Anomaly detected: {:?}", anomaly);
                });
        });
    })?;
    
    Ok(())
}

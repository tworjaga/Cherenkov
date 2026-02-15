use std::time::Duration;

pub struct TumblingWindow {
    size_secs: u64,
}

impl TumblingWindow {
    pub fn new(size_secs: u64) -> Self {
        Self { size_secs }
    }
    
    pub fn size(&self) -> Duration {
        Duration::from_secs(self.size_secs)
    }
}

pub struct WindowedStream<T> {
    buffer: Vec<T>,
    window_size: Duration,
    last_emit: std::time::Instant,
}

impl<T> WindowedStream<T> {
    pub fn new(window_size: Duration) -> Self {
        Self {
            buffer: Vec::new(),
            window_size,
            last_emit: std::time::Instant::now(),
        }
    }
    
    pub fn push(&mut self, item: T) -> Option<Vec<T>> {
        self.buffer.push(item);
        
        if self.last_emit.elapsed() >= self.window_size {
            self.last_emit = std::time::Instant::now();
            Some(std::mem::take(&mut self.buffer))
        } else {
            None
        }
    }
}

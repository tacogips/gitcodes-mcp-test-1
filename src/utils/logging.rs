use log::{Level, LevelFilter, Metadata, Record};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Custom logger implementation
pub struct SimpleLogger {
    level: LevelFilter,
}

impl SimpleLogger {
    /// Create a new SimpleLogger with the specified level
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }
    
    /// Initialize the logger
    pub fn init(level: LevelFilter) -> Result<(), log::SetLoggerError> {
        let logger = Self::new(level);
        log::set_max_level(level);
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = match record.level() {
                Level::Error => "ERROR",
                Level::Warn => "WARN ",
                Level::Info => "INFO ",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            };
            
            let now = chrono::Local::now();
            println!(
                "{} {} [{}] {}",
                now.format("%Y-%m-%d %H:%M:%S%.3f"),
                level_str,
                record.target(),
                record.args()
            );
        }
    }
    
    fn flush(&self) {}
}

/// An operation counter for logging statistics
pub struct OperationCounter {
    name: String,
    count: AtomicUsize,
    success_count: AtomicUsize,
    error_count: AtomicUsize,
}

impl OperationCounter {
    /// Create a new operation counter with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: AtomicUsize::new(0),
            success_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
        }
    }
    
    /// Increment the operation count
    pub fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::SeqCst) + 1
    }
    
    /// Record a successful operation
    pub fn record_success(&self) -> usize {
        self.success_count.fetch_add(1, Ordering::SeqCst) + 1
    }
    
    /// Record a failed operation
    pub fn record_error(&self) -> usize {
        self.error_count.fetch_add(1, Ordering::SeqCst) + 1
    }
    
    /// Get the current operation count
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
    
    /// Get the current success count
    pub fn success_count(&self) -> usize {
        self.success_count.load(Ordering::SeqCst)
    }
    
    /// Get the current error count
    pub fn error_count(&self) -> usize {
        self.error_count.load(Ordering::SeqCst)
    }
    
    /// Get the success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        let total = self.count();
        if total == 0 {
            return 1.0;
        }
        
        self.success_count() as f64 / total as f64
    }
    
    /// Get a summary of the counter statistics
    pub fn summary(&self) -> String {
        let total = self.count();
        let success = self.success_count();
        let error = self.error_count();
        let success_rate = self.success_rate() * 100.0;
        
        format!(
            "{}: total={}, success={}, error={}, success_rate={:.2}%",
            self.name, total, success, error, success_rate
        )
    }
    
    /// Log the counter summary at the specified level
    pub fn log_summary(&self, level: Level) {
        match level {
            Level::Error => log::error!("{}", self.summary()),
            Level::Warn => log::warn!("{}", self.summary()),
            Level::Info => log::info!("{}", self.summary()),
            Level::Debug => log::debug!("{}", self.summary()),
            Level::Trace => log::trace!("{}", self.summary()),
        }
    }
}
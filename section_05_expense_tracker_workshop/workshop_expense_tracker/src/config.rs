// Configuration constants and settings
use std::path::Path;

// File paths
pub const EXPENSES_FILE: &str = "expenses.csv";
pub const BACKUP_DIR: &str = "backups";

// CSV Settings
pub const CSV_HAS_HEADERS: bool = false;

// Date formats
pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const DISPLAY_DATE_FORMAT: &str = "%d/%m/%Y";

// Display settings
pub const TABLE_DATE_WIDTH: usize = 12;
pub const TABLE_TYPE_WIDTH: usize = 10;
pub const TABLE_AMOUNT_WIDTH: usize = 8;

// Validation constants
pub const MAX_CATEGORY_LENGTH: usize = 50;
pub const MAX_AMOUNT: f64 = 1_000_000.0;
pub const MIN_AMOUNT: f64 = 0.01;

// Utility functions
pub fn ensure_data_directory() -> std::io::Result<()> {
    if !Path::new(BACKUP_DIR).exists() {
        std::fs::create_dir(BACKUP_DIR)?;
    }
    Ok(())
}

// Get backup file
pub fn get_backup_filename() -> String {
    let now = chrono::Local::now();
    format!(
        "{}/expenses_backup_{}.csv",
        BACKUP_DIR,
        now.format("%Y%m%d_%H%M%S")
    )
}

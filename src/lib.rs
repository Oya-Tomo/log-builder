pub mod log;
pub mod logger;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use crate::{log::{Log, LogKind}, logger::{Logger, OutputTo}};

    #[test]
    fn print_log_example() {
        println!(
            "{}",
            Log::build(LogKind::Info, "PSQL", "created db and tables.".to_string(), Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Warn, "GCP-API", "session-key will be invalid soon.".to_string(), Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Error, "PSQL", "insert failed.".to_string(), Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Wip, "API", "user input invalid.".to_string(), Utc::now()).to_string_for_console()
        );
    }

    #[test]
    fn write_log_example() {
        let logger = Logger::build(
            vec![
                OutputTo::File("server.log".to_string()),
                OutputTo::Console,
            ]
        );

        logger.output(
            Log::build(LogKind::Info, "PSQL", "created db and tables.".to_string(), Utc::now())
        );
        logger.output(
            Log::build(LogKind::Warn, "API", "session-key will be invalid soon.".to_string(), Utc::now())
        );
        logger.output(
            Log::build(LogKind::Error, "PSQL", "insert failed.".to_string(), Utc::now())
        );
        logger.output(
            Log::build(LogKind::Wip, "SYSTEM", "user input invalid.".to_string(), Utc::now())
        );
    }
}

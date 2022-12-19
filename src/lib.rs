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
            Log::build(LogKind::Info, "PSQL", "created db and tables.", Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Warn, "GCP-API", "session-key will be invalid soon.", Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Error, "PSQL", "insert failed.", Utc::now()).to_string_for_console()
        );
        println!(
            "{}",
            Log::build(LogKind::Wip, "API", "user input invalid.", Utc::now()).to_string_for_console()
        );
    }

    #[test]
    fn write_log_example() {
        // Please create your original enum type;
        enum SystemNode {
            Server,
            PSQL,
            API,
        }

        // These strings will be printed as log in console.
        impl ToString for SystemNode  {
            fn to_string(&self) -> String {
                match self {
                    SystemNode::Server => "Server".to_string(),
                    SystemNode::PSQL => "PSQL".to_string(),
                    SystemNode::API => "API".to_string(),
                }
            }
        }

        let logger = Logger::build(
            vec![
                OutputTo::File("server.log".to_string()),
                OutputTo::Console,
            ]
        );

        logger.output(
            Log::build(LogKind::Info, SystemNode::PSQL, "created db and tables.", Utc::now())
        );
        logger.output(
            Log::build(LogKind::Warn, SystemNode::API, "session-key will be invalid soon.", Utc::now())
        );
        logger.output(
            Log::build(LogKind::Error, SystemNode::PSQL, "insert failed.", Utc::now())
        );
        logger.output(
            Log::build(LogKind::Wip, SystemNode::Server, "user input invalid.", Utc::now())
        );
    }
}

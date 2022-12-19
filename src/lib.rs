pub mod log;
pub mod logger;

#[cfg(test)]
mod tests {
    use crate::{log::Log, logger::{Logger, OutputTo}};

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
            Log::info(SystemNode::PSQL, "created db and tables.")
        );
        logger.output(
            Log::warn(SystemNode::API, "session-key will be invalid soon.")
        );
        logger.output(
            Log::error(SystemNode::PSQL, "insert failed.")
        );
        logger.output(
            Log::wip(SystemNode::Server, "user input invalid.")
        );
    }
}

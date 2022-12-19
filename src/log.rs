use chrono::{DateTime, Utc};

#[derive(Clone)]
pub enum LogKind {
    Info,
    Warn,
    Error,
    Wip,
}

impl LogKind {
    pub fn to_string_for_console(&self) -> String {
        match self {
            Self::Info  => "\x1b[32mINFO \x1b[m".to_string(),
            Self::Warn  => "\x1b[33mWARN \x1b[m".to_string(),
            Self::Error => "\x1b[31mERROR\x1b[m".to_string(),
            Self::Wip   => "\x1b[34mWIP  \x1b[m".to_string(),
        }
    }
}

impl ToString for LogKind {
    fn to_string(&self) -> String {
        match self {
            Self::Info  => "INFO ".to_string(),
            Self::Warn  => "WARN ".to_string(),
            Self::Error => "ERROR".to_string(),
            Self::Wip   => "WIP  ".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Log {
    pub kind: LogKind,
    pub node: String,
    pub message: String,
    pub time: DateTime<Utc>
}

impl Log {
    pub fn info<T: ToString>(node: T, message: &str) -> Self {
        return Log {
            kind: LogKind::Info,
            node: node.to_string(),
            message: message.to_string(),
            time: Utc::now(),
        };
    }

    pub fn warn<T: ToString>(node: T, message: &str) -> Self {
        return Log {
            kind: LogKind::Warn,
            node: node.to_string(),
            message: message.to_string(),
            time: Utc::now(),
        };
    }

    pub fn error<T: ToString>(node: T, message: &str) -> Self {
        return Log {
            kind: LogKind::Error,
            node: node.to_string(),
            message: message.to_string(),
            time: Utc::now(),
        };
    }

    pub fn wip<T: ToString>(node: T, message: &str) -> Self {
        return Log {
            kind: LogKind::Wip,
            node: node.to_string(),
            message: message.to_string(),
            time: Utc::now(),
        };
    }

    pub fn set_time(self, time: DateTime<Utc>) -> Self {
        let mut log = self.clone();
        log.time = time;
        return log;
    }

    pub fn to_string_for_file(&self) -> String {
        return format!(
            "{} ({}) [{}] {}\n",
            self.kind.to_string(),
            self.time.to_string(),
            self.node,
            self.message,
        );
    }

    pub fn to_string_for_console(&self) -> String {
        return format!(
            "{} ({}) [{}] {}",
            self.kind.to_string_for_console(),
            self.time.to_string(),
            self.node,
            self.message,
        );
    }
}

impl ToString for Log {
    fn to_string(&self) -> String {
        return format!(
            "{} ({}) [{}] {}",
            self.kind.to_string(),
            self.time.to_string(),
            self.node,
            self.message,
        );
    }
}
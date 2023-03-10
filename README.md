# Log Builder

<p align="center">
    <img src="./assets/logo.svg" alt="logo" title="Log-Builder" width=350>
</p>

## Classification of Program Log

| class | usage                                                                           |
| :---- | :------------------------------------------------------------------------------ |
| Info  | Print program information.                                                      |
| Warn  | Print program warning.                                                          |
| Error | This log is output from the system error location.                              |
| Wip   | This log is output from the part of the program that is currently being edited. |

## Usage

```rust
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
```

```
INFO  (2022-12-19 13:52:41.224424546 UTC) [PSQL] created db and tables.
WARN  (2022-12-19 13:52:41.224580092 UTC) [API] session-key will be invalid soon.
ERROR (2022-12-19 13:52:41.224615242 UTC) [PSQL] insert failed.
WIP   (2022-12-19 13:52:41.224646677 UTC) [Server] user input invalid.
```

## LICENSE : MIT

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
```

```
INFO  (2022-12-19 08:47:19.195638824 UTC) [PSQL] created db and tables.
WARN  (2022-12-19 08:47:19.196450856 UTC) [API] session-key will be invalid soon.
ERROR (2022-12-19 08:47:19.196663705 UTC) [PSQL] insert failed.
WIP   (2022-12-19 08:47:19.196790682 UTC) [SYSTEM] user input invalid.
```

## LICENSE : MIT

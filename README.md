# edge-application-event-api

The `api/` layer only (port contract) of `edge-application-event` — phase 1 of the `api`/`core`
split, see `edge-application#144`.

Ships `EventStore`/`EventBus`/`EventSource`/`EventPublisher`/`Aggregate`/`DomainEvent`, their
DTOs, and `EventAppendHandler`/`EventHistoryHandler` — generic `Handler` wiring types proving the
port composes with `edge-application-handler-api` without naming a concrete implementation. See
`edge-application#150`.

## Build and test

```sh
cargo build --all-targets
cargo test
```

See `bootstrap.sh` / `bootstrap.ps1` for the canonical setup command, and `examples/` for a
runnable demo.

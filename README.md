# coda

A REST API for querying music artist data from [MusicBrainz](https://musicbrainz.org/).

## Endpoints

### `GET /artists/{name}`

Search for artists by name. Returns the top results from the MusicBrainz database.

**Example:**
```
curl http://localhost:4000/artists/radiohead
```

## Running

```
cargo run
```

The server listens on `0.0.0.0:4000`.

## Dependencies

- [axum](https://github.com/tokio-rs/axum) — HTTP framework
- [reqwest](https://github.com/seanmonstar/reqwest) — HTTP client
- [tokio](https://tokio.rs/) — async runtime
- [serde](https://serde.rs/) — JSON serialization
- [anyhow](https://github.com/dtolnay/anyhow) — error handling
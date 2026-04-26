Concurrent URL Fetcher

An async Rust program that fetches multiple URLs concurrently using tokio and reqwest.

How to run:

cargo run

Example output:

Fetching 5 URLs concurrently...

Results:

URL - Status - Content-Type

https://www.rust-lang.org - 200 - text/html; charset=utf-8

https://www.github.com - 200 - text/html; charset=utf-8

https://www.google.com - 200 - text/html; charset=ISO-8859-1

Completed in: 1.07 seconds

Skills demonstrated:

    Async/await in Rust

    Concurrent HTTP requests with tokio

    Error handling

    Clean output formatting
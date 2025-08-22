# Read me

Uses the chromium tracing tools for visualisation

> [!IMPORTANT]
> You can view the result by opening
> [about://tracing/](about://tracing/) (preferred) or [https://ui.perfetto.dev/](https://ui.perfetto.dev/)

Should work on atleast all chromium based browsers.

## Features

- Enables tracing of a complex program
- Records timing of functions and scopes
- Low overhead
- Multithreading support
- function attribute and scope macros for convinience
- Counters
- Metadata (visible under the M on the top right on the about://tracing/ Website)

## Example Aplication

You can look at examples/simple example 

## Results

Resulting files form the tests are in results/

## Additional useful links

- <https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU>
- <http://src.chromium.org/viewvc/chrome/trunk/src/base/debug/trace_event_impl.cc?view=markup>
- <https://chromium.googlesource.com/chromium/src/+/refs/heads/main/chrome/tools/tracing/trace.html>
- <https://www.gamedeveloper.com/programming/in-depth-using-chrome-tracing-to-view-your-inline-profiling-data>

## TODO

- [ ] Markdown file
- [ ] doc comments
- [ ] compilation flag
- [ ] procmacro for other expressions
- [ ] What to do with:
  - [ ] headers?
    - [ ] Record header tag (add multiple recording into a single file?)
  - [ ] Value names
- [ ] Value JsonValue type?
- [ ] non static info?

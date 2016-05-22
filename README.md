# Go Rust Channel Benchmark
Compare performance of Go channels and goroutines with Rust channels and coroutines (mioco green threads), as well as Rust channels and standard Rust threads.

----------------
The benchmarks create a u64 channel, and then spawn some threads (10 and 10000),
and then each thread sends a u64 over the channel. Then we wait until all channel
messages have been received.

To run the benchmark, clone this repository and then run in a terminal: ./run.sh

----------------
My results on a quad-core AMD Phenom II 3GHz with 7GB RAM look like this:

Go:
BenchmarkGoChannel10uint64-4   	  100000	     12169 ns/op
BenchmarkGoChannel10000uint64-4	     200	  10815426 ns/op

Rust:
test tests::bench_rust_channel_10000_u64         ... bench: 182,032,841 ns/iter (+/- 13,512,699)
test tests::bench_rust_channel_10_u64            ... bench:   9,514,493 ns/iter (+/- 6,558,879)
test tests::bench_rust_channel_threads_10000_u64 ... bench: 505,942,488 ns/iter (+/- 28,496,670)
test tests::bench_rust_channel_threads_10_u64    ... bench:     306,669 ns/iter (+/- 71,823)

----------------
I wrote this because I'm looking to switch from Go to Rust for new projects,
and I wanted to see how the two languages compare in regards to concurrency
performance.

I'm also looking for any feedback about how I can do concurrency in Rust in a
way that outperforms (or matches the performance of) Go.

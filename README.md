# Examples for testing coc-rust-analyzer debugging

The whole point of this repo is providing some sample code for testing
the coc-rust-analyzer debug feature. The rust files contain various
functions which can be run in the debugger.

Because debuggers need to know which executable they should run,
coc-rust-analyzer's debugging command passes the filename to the debugger.
However, at the time of writing, this does not work properly.
The corresponding [PR](https://github.com/fannheyward/coc-rust-analyzer/pull/1055)
contains more details.

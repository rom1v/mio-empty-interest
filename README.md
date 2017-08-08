This is a minimal sample to test polling with empty interest on several
platforms.

See: [Allow registration with empty interest][pr].

[pr]: https://github.com/carllerche/mio/pull/640

## Principle

This sample starts listening on a (blocking) TCP socket on 127.0.0.1:1234 in a
separate thread.

Then it connects to it using a (non-blocking / mio) TCP socket.

The server sends a number to the client every second. The client polls to get
the incoming events.

Initially, it polls for _readable_ events twice, so it retrieves the first two
numbers). Then it polls with *empty interest* set with a timeout of ~5 seconds,
so it temporarily ignores the 5 following numbers. Then it polls again for
_readable_ events, so it retrieves all the number that have been ignored at
once.


## Execute

Note that it builds `mio` from
<https://github.com/rom1v/mio/commits/empty_interest>.

Compile with _Cargo_:

    cargo build

Then run the generated binary:

    target/debug/mio-empty-interest

The expected output is:

```
interest: {readable}
0 -->
      --> 0
1 -->
      --> 1
interest: {}
2 -->
3 -->
4 -->
5 -->
6 -->
interest: {readable}
      --> 23456
7 -->
      --> 7
8 -->
      --> 8
```

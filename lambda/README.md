Check stack trace with Rust closures

```
$ cargo run
   Compiling lambda v0.1.0 (file:///Users/yannsimon/projects/rust/rust-playground/lambda)
     Running `target/debug/lambda`
thread '<main>' panicked at 'attempted to divide by zero', ../src/libcore/ops.rs:369
note: Run with `RUST_BACKTRACE=1` for a backtrace.
Process didn't exit successfully: `target/debug/lambda` (exit code: 101)
```

```
$ RUST_BACKTRACE=1 cargo run
     Running `target/debug/lambda`
thread '<main>' panicked at 'attempted to divide by zero', ../src/libcore/ops.rs:369
stack backtrace:
   1:        0x10a005d38 - sys::backtrace::tracing::imp::write::h3b28049ffa6d6406ZZu
   2:        0x10a007155 - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.43942
   3:        0x10a006d78 - panicking::default_handler::h9792cd244a79d9daorz
   4:        0x10a0008c6 - sys_common::unwind::begin_unwind_inner::h127fbab41243a988LYt
   5:        0x10a000b0e - sys_common::unwind::begin_unwind_fmt::h849e306453bce5feRXt
   6:        0x10a005257 - rust_begin_unwind
   7:        0x10a02a460 - panicking::panic_fmt::h98b8cbb286f5298alcM
   8:        0x10a02a75c - panicking::panic::h4265c0105caa1121SaM
   9:        0x109fffc14 - ops::i32.Div::div::hd1bfdcbc45a2f6b38hs
  10:        0x109fffbcc - ops::_&'a i32.Div<i32>::div::hc6fe8e5bf6d64d7ctis
  11:        0x109ffebde - main::_$u7b$$u7b$closure$u7d$$u7d$::closure.4584
  12:        0x109ffebac - ops::impls::_&'a mut F.FnOnce<A>::call_once::h13474404769251525136
  13:        0x109ffeb51 - option::Option<T>::map::h5198165658577630433
  14:        0x109ffeafc - iter::Map<I, F>.Iterator::next::h12415006720423021653
  15:        0x109ffe83b - vec::Vec<T>.FromIterator<T>::from_iter::h17385690455391238518
  16:        0x109ffe7b9 - iter::Iterator::collect::h11162526020885055196
  17:        0x109ffd976 - main::hf103c959446ec84beaa
  18:        0x10a006982 - sys_common::unwind::try::try_fn::h6504634030632813459
  19:        0x10a0051eb - __rust_try
  20:        0x10a006840 - rt::lang_start::h9c1978904cd16d9cujz
  21:        0x109ffff59 - main
Process didn't exit successfully: `target/debug/lambda` (exit code: 101)
```

Not so good...
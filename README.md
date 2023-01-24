Sample repo showing that a simple usage of the cc crate doesn't work
properly with esp-rs.  Configured to use esp32c3 target (RISC-V) but the same
problem occurs for esp32 (Xtensa), though the errors are slightly different.

Error is:

```
  running: "cc" "-Os" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-march=rv32imc" "-mabi=ilp32" "-mcmodel=medany" "-Wall" "-Wextra" "-o" "/media/jasta/data/software/cc-broken/target/riscv32imc-esp-espidf/debug/build/cc-broken-28576ab2a358ec5f/out/src/doubler.o" "-c" "src/doubler.c"
  cargo:warning=cc: error: unrecognized argument in option ‘-mabi=ilp32’
  cargo:warning=cc: note: valid arguments to ‘-mabi=’ are: ms sysv
  cargo:warning=cc: error: unrecognized argument in option ‘-mcmodel=medany’
  cargo:warning=cc: note: valid arguments to ‘-mcmodel=’ are: 32 kernel large medium small
  exit status: 1

  --- stderr


  error occurred: Command "cc" "-Os" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-march=rv32imc" "-mabi=ilp32" "-mcmodel=medany" "-Wall" "-Wextra" "-o" "/media/jasta/data/software/cc-broken/target/riscv32imc-esp-espidf/debug/build/cc-broken-28576ab2a358ec5f/out/src/doubler.o" "-c" "src/doubler.c" with args "cc" did not execute successfully (status code exit status: 1).
```

Which suggests to me that the wrong cc is being invoked (host gcc), though I can't figure out why.  TARGET is getting set correctly.  Does the same thing whether `. ~/export-idf.sh` is invoked or not (which I understand is only relevant for Xtensa building anyway).

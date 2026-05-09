; AIAL-generated LLVM IR
target triple = "x86_64-unknown-linux-gnu"


define i32 @main() {

b0:
  %v0 = add i64 0, 42
  %trunc0 = trunc i64 %v0 to i32
  ret i32 %trunc0
}


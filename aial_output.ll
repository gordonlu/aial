; AIAL-generated LLVM IR
target triple = "x86_64-unknown-linux-gnu"


define void @main() {

b0:
  %v0 = add i64 0, 0
  %v1 = alloca i64
  store i64 %v0, i64* %v1
  %v2 = add i64 0, 0
  %v3 = add i64 0, 1000
  %v4 = alloca i64
  %v5 = alloca i64
  %v6 = add i64 0, 0
  store i64 %v6, i64* %v4
  %v7 = add i64 0, 0
  store i64 %v3, i64* %v5
  %v8 = add i64 0, 0
  br label %b1

b1:
  %v9 = load i64, i64* %v4
  %v10 = load i64, i64* %v5
  %v11 = icmp slt i64 %v9, %v10
  br i1 i64 0, label %b2, label %b4

b2:
  %v12 = load i64, i64* %v4
  %v13 = alloca i64
  store i64 %v12, i64* %v13
  %v14 = add i64 0, 0
  %v15 = load i64, i64* %v1
  %v16 = add i64 0, 1
  %v17 = add i64 %v15, %v16
  store i64 0, i64* %v1
  %v18 = add i64 0, 0
  br label %b3

b3:
  %v19 = load i64, i64* %v4
  %v20 = add i64 0, 1
  %v21 = add i64 %v19, %v20
  store i64 0, i64* %v4
  %v22 = add i64 0, 0
  br label %b1

b4:
  ret void
}


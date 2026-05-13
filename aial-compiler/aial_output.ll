; AIAL-generated LLVM IR
target triple = "x86_64-unknown-linux-gnu"

declare i64 @aial_rt_map_new()
declare void @aial_rt_map_set(i64, i64, i64)
declare i64 @aial_rt_map_has(i64, i64)
declare void @aial_rt_println(i64)
declare void @aial_rt_string_register(i64, i8*)

@.str0 = private unnamed_addr constant [2 x i8] c"k\00", align 1
@.str1 = private unnamed_addr constant [2 x i8] c"v\00", align 1
@.str2 = private unnamed_addr constant [2 x i8] c"k\00", align 1
@.str3 = private unnamed_addr constant [15 x i8] c"has == 1 works\00", align 1
@.str4 = private unnamed_addr constant [8 x i8] c"missing\00", align 1
@.str5 = private unnamed_addr constant [19 x i8] c"not has != 1 works\00", align 1

define i32 @main() {

b0:
  %str_init_0 = getelementptr inbounds [2 x i8], [2 x i8]* @.str0, i32 0, i32 0
  call void @aial_rt_string_register(i64 0, i8* %str_init_0)
  %str_init_1 = getelementptr inbounds [2 x i8], [2 x i8]* @.str1, i32 0, i32 0
  call void @aial_rt_string_register(i64 1, i8* %str_init_1)
  %str_init_2 = getelementptr inbounds [2 x i8], [2 x i8]* @.str2, i32 0, i32 0
  call void @aial_rt_string_register(i64 2, i8* %str_init_2)
  %str_init_3 = getelementptr inbounds [15 x i8], [15 x i8]* @.str3, i32 0, i32 0
  call void @aial_rt_string_register(i64 3, i8* %str_init_3)
  %str_init_4 = getelementptr inbounds [8 x i8], [8 x i8]* @.str4, i32 0, i32 0
  call void @aial_rt_string_register(i64 4, i8* %str_init_4)
  %str_init_5 = getelementptr inbounds [19 x i8], [19 x i8]* @.str5, i32 0, i32 0
  call void @aial_rt_string_register(i64 5, i8* %str_init_5)
  %v0 = call i64 @aial_rt_map_new()
  %aptr1 = alloca i64
  %v1 = ptrtoint i64* %aptr1 to i64
  %sptr2 = inttoptr i64 %v1 to i64*
  store i64 %v0, i64* %sptr2
  %v2 = add i64 0, 0
  %lptr3 = inttoptr i64 %v1 to i64*
  %v3 = load i64, i64* %lptr3
  %v4 = add i64 0, 0
  %v5 = add i64 0, 1
  call void @aial_rt_map_set(i64 %v3, i64 %v4, i64 %v5)
  %v6 = add i64 0, 0
  %lptr7 = inttoptr i64 %v1 to i64*
  %v7 = load i64, i64* %lptr7
  %v8 = add i64 0, 0
  %x9 = call i64 @aial_rt_map_has(i64 %v7, i64 %v8)
  %v9 = trunc i64 %x9 to i1
  %v10 = add i64 0, 1
  %zext9 = zext i1 %v9 to i64
  %v11 = icmp eq i64 %zext9, %v10
  br i1 %v11, label %b1, label %b2

b1:
  %v12 = add i64 0, 3
  call void @aial_rt_println(i64 %v12)
  %v13 = add i64 0, 0
  br label %b3

b3:
  %lptr14 = inttoptr i64 %v1 to i64*
  %v14 = load i64, i64* %lptr14
  %v15 = add i64 0, 4
  %x16 = call i64 @aial_rt_map_has(i64 %v14, i64 %v15)
  %v16 = trunc i64 %x16 to i1
  %v17 = add i64 0, 1
  %zext16 = zext i1 %v16 to i64
  %v18 = icmp ne i64 %zext16, %v17
  br i1 %v18, label %b4, label %b5

b4:
  %v19 = add i64 0, 5
  call void @aial_rt_println(i64 %v19)
  %v20 = add i64 0, 0
  br label %b6

b2:
  br label %b3

b5:
  br label %b6

b6:
  ret i32 0
}


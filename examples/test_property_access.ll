; ModuleID = 'program'
source_filename = "program"

@string_literal = global [6 x i8] c"valor\00"
@string_literal.1 = global [21 x i8] c"Property access test\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %object_alloc = call ptr @malloc(i64 24)
  store i64 1, ptr %object_alloc, align 4
  %obj_key_ptr_0 = getelementptr i64, ptr %object_alloc, i64 1
  store i64 ptrtoint (ptr @string_literal to i64), ptr %obj_key_ptr_0, align 4
  %obj_value_ptr_0 = getelementptr i64, ptr %object_alloc, i64 2
  store i64 42, ptr %obj_value_ptr_0, align 4
  %obj = alloca ptr, align 8
  store ptr %object_alloc, ptr %obj, align 8
  %obj1 = load ptr, ptr %obj, align 8
  %object_size = load i64, ptr %obj1, align 4
  %counter = alloca i64, align 8
  store i64 0, ptr %counter, align 4
  br label %prop_search

prop_search:                                      ; preds = %entry
  %load_counter = load i64, ptr %counter, align 4
  %is_end = icmp eq i64 %load_counter, %object_size
  br i1 %is_end, label %prop_not_found, label %prop_found

prop_found:                                       ; preds = %prop_search
  %x = alloca i64, align 8
  store i64 42, ptr %x, align 4
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.1)
  ret i64 0

prop_not_found:                                   ; preds = %prop_search
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)

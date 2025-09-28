; ModuleID = 'program'
source_filename = "program"

@string_literal = global [6 x i8] c"width\00"
@string_literal.1 = global [7 x i8] c"height\00"
@string_literal.2 = global [29 x i8] c"Object created successfully!\00"
@format_str = global [4 x i8] c"%s\0A\00"

define i64 @main() {
entry:
  %object_alloc = call ptr @malloc(i64 40)
  store i64 2, ptr %object_alloc, align 4
  %obj_key_ptr_0 = getelementptr i64, ptr %object_alloc, i64 1
  store i64 ptrtoint (ptr @string_literal to i64), ptr %obj_key_ptr_0, align 4
  %obj_value_ptr_0 = getelementptr i64, ptr %object_alloc, i64 2
  store i64 800, ptr %obj_value_ptr_0, align 4
  %obj_key_ptr_1 = getelementptr i64, ptr %object_alloc, i64 3
  store i64 ptrtoint (ptr @string_literal.1 to i64), ptr %obj_key_ptr_1, align 4
  %obj_value_ptr_1 = getelementptr i64, ptr %object_alloc, i64 4
  store i64 600, ptr %obj_value_ptr_1, align 4
  %config = alloca ptr, align 8
  store ptr %object_alloc, ptr %config, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.2)
  ret i64 0
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)

; ModuleID = 'program'
source_filename = "program"

@string_literal = global [6 x i8] c"width\00"
@string_literal.1 = global [7 x i8] c"height\00"
@string_literal.2 = global [8 x i8] c"version\00"
@string_literal.3 = global [27 x i8] c"Config criado com sucesso!\00"
@format_str = global [4 x i8] c"%s\0A\00"
@string_literal.4 = global [8 x i8] c"Width: \00"
@format_str.5 = global [6 x i8] c"%lld\0A\00"

define i64 @main() {
entry:
  %object_alloc = call ptr @malloc(i64 56)
  store i64 3, ptr %object_alloc, align 4
  %obj_key_ptr_0 = getelementptr i64, ptr %object_alloc, i64 1
  store i64 ptrtoint (ptr @string_literal to i64), ptr %obj_key_ptr_0, align 4
  %obj_value_ptr_0 = getelementptr i64, ptr %object_alloc, i64 2
  store i64 800, ptr %obj_value_ptr_0, align 4
  %obj_key_ptr_1 = getelementptr i64, ptr %object_alloc, i64 3
  store i64 ptrtoint (ptr @string_literal.1 to i64), ptr %obj_key_ptr_1, align 4
  %obj_value_ptr_1 = getelementptr i64, ptr %object_alloc, i64 4
  store i64 600, ptr %obj_value_ptr_1, align 4
  %obj_key_ptr_2 = getelementptr i64, ptr %object_alloc, i64 5
  store i64 ptrtoint (ptr @string_literal.2 to i64), ptr %obj_key_ptr_2, align 4
  %obj_value_ptr_2 = getelementptr i64, ptr %object_alloc, i64 6
  store i64 1, ptr %obj_value_ptr_2, align 4
  %config = alloca ptr, align 8
  store ptr %object_alloc, ptr %config, align 8
  %printf_str_call = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.3)
  %config1 = load ptr, ptr %config, align 8
  %object_size = load i64, ptr %config1, align 4
  %counter = alloca i64, align 8
  store i64 0, ptr %counter, align 4
  br label %prop_search

prop_search:                                      ; preds = %entry
  %load_counter = load i64, ptr %counter, align 4
  %is_end = icmp eq i64 %load_counter, %object_size
  br i1 %is_end, label %prop_not_found, label %prop_found

prop_found:                                       ; preds = %prop_search
  %w = alloca i64, align 8
  store i64 42, ptr %w, align 4
  %printf_str_call2 = call i64 (ptr, ...) @printf(ptr @format_str, ptr @string_literal.4)
  %w3 = load i64, ptr %w, align 4
  %printf_int_call = call i64 (ptr, ...) @printf(ptr @format_str.5, i64 %w3)
  ret i64 0

prop_not_found:                                   ; preds = %prop_search
}

declare ptr @malloc(i64)

declare i64 @printf(ptr, ...)

; Kotlin compress query - extract top-level declarations
; Note: compress.rs skips overlapping (nested) captures, so we only
; capture the outermost declaration nodes.
(class_declaration) @class

(function_declaration) @func

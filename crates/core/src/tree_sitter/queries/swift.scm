; Swift compress query - extract top-level declarations
; compress.rs skips overlapping (nested) captures, so we only
; capture the outermost declaration nodes.
(class_declaration
  name: (type_identifier) @name
) @class

(function_declaration
  name: (simple_identifier) @name
) @func

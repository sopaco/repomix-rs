; Dart compress query - extract function/class signatures
(function_signature
  name: (identifier) @name
  (formal_parameter_list) @params
)

(method_signature
  (function_signature
    name: (identifier) @name
    (formal_parameter_list) @params
  )
)

(class_definition
  name: (identifier) @name
  body: (class_body) @body
)

(enum_declaration
  name: (identifier) @name
)

(mixin_declaration
  name: (identifier) @name
)

(extension_declaration
  name: (identifier)? @name
)

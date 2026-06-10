; C++ compress query - extract function/class signatures
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name
    parameters: (parameter_list) @params
  )
)

(class_specifier
  name: (type_identifier) @name
  body: (field_declaration_list) @body
)

(struct_specifier
  name: (type_identifier) @name
  body: (field_declaration_list) @body
)

(namespace_definition
  name: (namespace_identifier) @name
  body: (declaration_list) @body
)

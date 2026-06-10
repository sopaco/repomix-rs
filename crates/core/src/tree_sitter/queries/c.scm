; C compress query - extract function/struct signatures
(function_definition
  type: (_) @return_type
  declarator: (function_declarator
    declarator: (identifier) @name
    parameters: (parameter_list) @params
  )
)

(struct_specifier
  name: (type_identifier) @name
  body: (field_declaration_list) @body
)

(type_definition
  type: (_) @type
  declarator: (type_identifier) @name
)

(declaration
  type: (_) @type
  declarator: (function_declarator
    declarator: (identifier) @name
    parameters: (parameter_list) @params
  )
)
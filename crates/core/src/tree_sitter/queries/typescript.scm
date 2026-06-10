; TypeScript compress query - extract function/class signatures
(function_declaration
  name: (identifier) @name
  parameters: (formal_parameters) @params
  return_type: (type_annotation)? @return_type
)

(class_declaration
  name: (type_identifier) @name
  body: (class_body) @body
)

(method_definition
  name: (property_identifier) @name
  parameters: (formal_parameters) @params
)

(export_statement
  declaration: (function_declaration) @func
)

(export_statement
  declaration: (class_declaration) @class
)

(interface_declaration
  name: (type_identifier) @name
  body: (interface_body) @body
)

(type_alias_declaration
  name: (type_identifier) @name
  value: (_) @value
)
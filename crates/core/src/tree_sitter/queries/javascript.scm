; JavaScript compress query - extract function/class signatures
(function_declaration
  name: (identifier) @name
  parameters: (formal_parameters) @params
)

(class_declaration
  name: (identifier) @name
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

(arrow_function
  parameters: (formal_parameters) @params
)
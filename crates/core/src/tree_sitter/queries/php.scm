; PHP compress query - extract function/class definitions
(function_definition
  name: (name) @name
  parameters: (formal_parameters) @params
  body: (compound_statement) @body
)

(class_declaration
  name: (name) @name
  body: (declaration_list) @body
)

(interface_declaration
  name: (name) @name
  body: (declaration_list) @body
)

(method_declaration
  name: (name) @name
  parameters: (formal_parameters) @params
)

(function_call_expression
  function: (_) @func
  arguments: (arguments) @params
)
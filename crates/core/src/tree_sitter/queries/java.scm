; Java compress query - extract class/method signatures
(class_declaration
  name: (identifier) @name
  body: (class_body) @body
)

(method_declaration
  name: (identifier) @name
  parameters: (formal_parameters) @params
  type: (type_identifier)? @return_type
)

(interface_declaration
  name: (identifier) @name
  body: (interface_body) @body
)

(enum_declaration
  name: (identifier) @name
  body: (enum_body) @body
)

(constructor_declaration
  name: (identifier) @name
  parameters: (formal_parameters) @params
)
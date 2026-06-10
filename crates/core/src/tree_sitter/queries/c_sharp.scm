; C# compress query - extract method/class definitions
(method_declaration
  name: (identifier) @name
  parameters: (parameter_list) @params
)

(class_declaration
  name: (identifier) @name
)

(interface_declaration
  name: (identifier) @name
)

(struct_declaration
  name: (identifier) @name
)

(namespace_declaration
  name: (identifier) @name
)

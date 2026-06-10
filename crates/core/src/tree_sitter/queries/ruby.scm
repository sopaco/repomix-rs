; Ruby compress query - extract method/class definitions
(method
  name: (identifier) @name
  parameters: (method_parameters) @params
)

(singleton_method
  object: (_) @object
  name: (identifier) @name
  parameters: (method_parameters) @params
)

(class
  name: (constant) @name
)

(module
  name: (constant) @name
)

; Python compress query - extract function/class signatures
(function_definition
  name: (identifier) @name
  parameters: (parameters) @params
  return_type: (type)? @return_type
)

(class_definition
  name: (identifier) @name
  body: (block) @body
)

(decorated_definition
  definition: (function_definition) @func
)

(decorated_definition
  definition: (class_definition) @class
)
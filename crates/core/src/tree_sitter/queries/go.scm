; Go compress query - extract function/type signatures
(function_declaration
  name: (identifier) @name
  parameters: (parameter_list) @params
)

(method_declaration
  receiver: (parameter_list) @receiver
  name: (field_identifier) @name
  parameters: (parameter_list) @params
)

(type_declaration
  (type_spec
    name: (type_identifier) @name
    type: (struct_type
      (field_declaration_list) @body
    )
  )
)

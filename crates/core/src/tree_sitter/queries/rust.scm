; Rust compress query - extract function/struct/impl signatures
(function_item
  name: (identifier) @name
  parameters: (parameters) @params
)

(struct_item
  name: (type_identifier) @name
  body: (field_declaration_list) @body
)

(impl_item
  type: (type_identifier) @name
  body: (declaration_list) @body
)

(trait_item
  name: (type_identifier) @name
  body: (declaration_list) @body
)

(enum_item
  name: (type_identifier) @name
  body: (enum_variant_list) @body
)

(macro_definition
  name: (identifier) @name
)

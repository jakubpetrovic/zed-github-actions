; Top-level keys: name, on, env, jobs, ...
(document
  (block_node
    (block_mapping
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @name))) @item)))

; Jobs
(document
  (block_node
    (block_mapping
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @_jobs))
        value: (block_node
          (block_mapping
            (block_mapping_pair
              key: (flow_node
                (plain_scalar
                  (string_scalar) @name))) @item))
        (#eq? @_jobs "jobs")))))

; Named steps
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @_steps))
  value: (block_node
    (block_sequence
      (block_sequence_item
        (block_node
          (block_mapping
            (block_mapping_pair
              key: (flow_node
                (plain_scalar
                  (string_scalar) @_name))
              value: (flow_node
                (plain_scalar
                  (string_scalar) @name))
              (#eq? @_name "name"))))) @item))
  (#eq? @_steps "steps"))

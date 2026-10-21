; Step run: scripts are bash. The block scalar includes its "|" header; bash recovers.
(block_sequence_item
  (block_node
    (block_mapping
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @_key))
        value: (block_node
          (block_scalar) @injection.content)
        (#eq? @_key "run")
        (#set! injection.language "bash")))))

(block_sequence_item
  (block_node
    (block_mapping
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @_key))
        value: (flow_node
          (plain_scalar
            (string_scalar) @injection.content))
        (#eq? @_key "run")
        (#set! injection.language "bash")))))

; actions/github-script: "uses" must come before "with"
(block_sequence_item
  (block_node
    (block_mapping
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @_uses_key))
        value: (flow_node
          (plain_scalar
            (string_scalar) @_uses)))
      (block_mapping_pair
        key: (flow_node
          (plain_scalar
            (string_scalar) @_with_key))
        value: (block_node
          (block_mapping
            (block_mapping_pair
              key: (flow_node
                (plain_scalar
                  (string_scalar) @_script_key))
              value: (block_node
                (block_scalar) @injection.content)))))
      (#eq? @_uses_key "uses")
      (#match? @_uses "^actions/github-script@")
      (#eq? @_with_key "with")
      (#eq? @_script_key "script")
      (#set! injection.language "javascript"))))

; === Standard YAML highlights ===

(comment) @comment

(boolean_scalar) @boolean
(null_scalar) @constant.builtin
(integer_scalar) @number
(float_scalar) @number

(double_quote_scalar) @string
(single_quote_scalar) @string
(block_scalar) @string
(string_scalar) @string

(escape_sequence) @string.escape

; Anchors and aliases
(anchor_name) @type
(alias_name) @type
(tag) @type

; Punctuation
["," "-" ":" ">" "?" "|"] @punctuation.delimiter
["[" "]" "{" "}"] @punctuation.bracket
["*" "&" "---" "..."] @punctuation.special

; === GitHub Actions-specific highlights ===

; Generic map keys
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @variable.member)))

; Top-level and structural workflow keys
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @keyword))
  (#match? @keyword "^(on|jobs|steps|uses|with|env|needs|if|run|name|runs-on|permissions|strategy|matrix|outputs|defaults|concurrency|secrets|inputs|shell|services|container|environment)$"))

; Event trigger keys
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @keyword.coroutine))
  (#match? @keyword.coroutine "^(push|pull_request|pull_request_target|workflow_dispatch|workflow_call|workflow_run|schedule|release|deployment|create|delete|fork|issue_comment|issues|label|milestone|page_build|project|public|registry_package|repository_dispatch|status|watch|check_run|check_suite|deployment_status|discussion|discussion_comment|merge_group|pull_request_review|pull_request_review_comment|registry_package|status|watch)$"))

; Action metadata keys
(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @attribute))
  (#match? @attribute "^(branches|branches-ignore|tags|tags-ignore|paths|paths-ignore|types|ref|repository|token|fail-fast|include|exclude|max-parallel|timeout-minutes|continue-on-error|working-directory|id|description|required|default|type|value|options|deprecationMessage)$"))

; Strings containing GitHub Actions expressions ${{ }} — mark the whole string
; so they stand out from plain static strings.
; True sub-expression highlighting requires a custom injected grammar (v0.2.0).
((double_quote_scalar) @string.special
  (#match? @string.special "\\$\\{\\{"))
((single_quote_scalar) @string.special
  (#match? @string.special "\\$\\{\\{"))
((string_scalar) @string.special
  (#match? @string.special "\\$\\{\\{"))
((block_scalar) @string.special
  (#match? @string.special "\\$\\{\\{"))

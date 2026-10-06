; Solidity test detection for Zed runnables.
;
; Captures without a leading underscore are exposed to tasks as
; $ZED_CUSTOM_<name>, e.g. $ZED_CUSTOM_contract holds the enclosing contract
; name so tasks can pass `--match-contract` alongside `--match-test`.

; Test functions (test*, testFuzz*, testFork*, invariant*, statefulFuzz*)
(
  (contract_declaration
    name: (identifier) @contract
    body: (contract_body
      (function_definition
        name: (identifier) @run
        (#match? @run "^(test|invariant|statefulFuzz)")) @_))
  (#set! tag solidity-test)
)

; Contracts that contain at least one test function (run the whole suite).
; Plain source contracts get no run button.
(
  (contract_declaration
    name: (identifier) @run
    body: (contract_body
      (function_definition
        name: (identifier) @_test_fn
        (#match? @_test_fn "^(test|invariant|statefulFuzz)")))) @_
  (#set! tag solidity-contract-test)
)

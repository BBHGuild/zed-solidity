; Solidity test function detection for Zed editor

; Match test functions (test*, testFail*, testFuzz*, testFork*, invariant*, statefulFuzz*)
(
  (
    (function_definition
      name: (identifier) @run
      (#match? @run "^(test|invariant|statefulFuzz)"))
  ) @_
  (#set! tag solidity-test)
)

; Match contracts for running all tests in a contract
(
  (contract_declaration
    name: (identifier) @run
  ) @_
  (#set! tag solidity-contract-test)
)

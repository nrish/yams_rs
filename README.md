just a simple math solver/parser written in rust.

general design structure

- two base nodes, functions and operators
- functions are themselves their own statements, and are parsed separately.

steps in parsing

- read statement string into tree
- divide statement by order of operations into first and second parts
- keep dividing until no more operators are left
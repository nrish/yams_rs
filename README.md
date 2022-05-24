just a simple math solver/parser written in rust.

general design structure

- Nodes are things which need resolving (functions, variables), or operators which contain more nodes.

steps in parsing

- validate parenthesis
- divide statement by order of operations into left and right parts, respecting parenthesis
- keep dividing until no more operators are left

currently just sets up the tree, would be really simple to implement evaluation though.
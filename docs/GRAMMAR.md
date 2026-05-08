# AAL 语法规范 (EBNF)

## 关键字
`fn let mut return if else match as type struct enum trait impl ask context tool use test receive self for in while loop break continue true false null`

## 操作符优先级（低 → 高）
`|>` → `||` → `&&` → `== !=` → `< > <= >=` → `+ -` → `* / %` → 前缀 `! -`

## 词法补充
- 标识符：`[a-zA-Z_][a-zA-Z0-9_]*`
- 整数字面量：`[0-9][0-9_]*`
- 浮点字面量：`[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?`
- 字符串转义：`\n \t \r \0 \" \\ \xNN \u{NNNN}`
- 注释：`// ...` 行注释，`/* ... */` 块注释

## 语法规则（精简版）


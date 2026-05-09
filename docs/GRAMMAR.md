# AIAL Language Specification (Frozen)

> Version 1.0 — Language Freeze
> Changes after this point require RFC process.

## 1. Keywords

```
ask as break continue else enum fn for if impl in
let loop match mut null receive return self struct
test trait true false type use while
```

## 2. Operators & Precedence (low → high)

| Level | Operators | Assoc |
|-------|-----------|-------|
| 1 | `\|>` (pipe) | left |
| 2 | `\|\|` (or) | left |
| 3 | `&&` (and) | left |
| 4 | `==` `!=` (equality) | left |
| 5 | `<` `>` `<=` `>=` (comparison) | left |
| 6 | `+` `-` (add, sub) | left |
| 7 | `*` `/` `%` (mul, div, rem) | left |
| 8 | prefix `!` `-` (not, negate) | right |

## 3. Lexical Rules

- Identifier: `[a-zA-Z_][a-zA-Z0-9_]*`
- Integer: `[0-9][0-9_]*`
- Float: `[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?`
- String escapes: `\n \t \r \0 \" \\ \xNN \u{NNNN}`
- Comments: `//` line, `/* */` block (nestable)

## 4. Syntax (EBNF)

```
Program         = { TopLevelItem } [ MainFn ] ;
TopLevelItem    = FnDef | StructDef | EnumDef | TypeAlias | TraitDef
                | ImplBlock | UseStmt | TestDef ;
MainFn          = "fn" "main" "(" ")" Block ;

FnDef           = [ Attributes ] "fn" Ident [ GenericParams ]
                  "(" Params ")" [ "->" Type ] Block ;
Params          = [ Param { "," Param } ] ;
Param           = [ "mut" ] Ident ":" Type [ "=" Expr ] ;

Block           = "{" { Stmt } [ TrailExpr ] "}" ;
Stmt            = LetStmt | AssignStmt | ReturnStmt
                | IfStmt | MatchStmt | ForStmt | WhileStmt | LoopStmt
                | BreakStmt | ContinueStmt | ExprStmt ;
TrailExpr       = Expr ;  (* without trailing semicolon *)

LetStmt         = "let" [ "mut" ] Ident [ ":" Type ] "=" Expr ";" ;
AssignStmt      = LValue "=" Expr ";" ;
ReturnStmt      = "return" [ Expr ] ";" ;
IfStmt          = "if" Expr Block [ "else" ( IfStmt | Block ) ] ;
MatchStmt       = "match" Expr "{" { MatchArm } "}" ;
MatchArm        = Pattern "=>" ( Block | Expr [","] ) ;
ForStmt         = "for" Ident "in" Expr Block ;
WhileStmt       = "while" Expr Block ;
LoopStmt        = "loop" Block ;
BreakStmt       = "break" ";" ;
ContinueStmt    = "continue" ";" ;
ExprStmt        = Expr ";" ;

Expr            = ExprBin ;
ExprBin         = ExprUnary { BinOp ExprUnary } ;
ExprUnary       = { "!" | "-" } ExprPrimary ;
ExprPrimary     = Literal | Ident | SelfExpr | AskExpr | ReceiveExpr
                | BlockExpr | IfExpr | MatchExpr
                | "(" Expr ")" | Path
                | Expr "." Ident                   (* field access *)
                | Expr "." Ident "(" Args ")"      (* method call *)
                | Expr "[" Expr "]"                (* indexing *)
                | Path "{" FieldList "}"            (* struct literal *) ;
Literal         = IntLiteral | FloatLiteral | StringLiteral
                | BoolLiteral | NullLiteral ;

AskExpr         = "ask" "(" [ AskOptions ] ")"
                | "ask" "." "many" "(" "[" [ AskGroup { "," AskGroup } ] "]" ")"
                | "ask" "." "race" "(" "[" [ AskGroup { "," AskGroup } ] "]" ")" ;
AskOptions      = AskOption { "," AskOption } ;
AskOption       = [ Ident "=" ] Expr ;  (* bare expr treated as prompt *)
AskGroup        = "(" AskOptions ")" ;

Pattern         = Ident | "_" | Literal | Path "(" Pattern { "," Pattern } ")"
                | Pattern "|" Pattern | Pattern "as" Ident ;

Type            = BaseType | Path [ "<" Type { "," Type } ">" ]
                | "fn" "(" [ Type { "," Type } ] ")" [ "->" Type ]
                | "[" Type ";" IntLiteral "]" | "[" Type "]" 
                | Type "?" | Type "|" Type ;
BaseType        = "int" | "int8" | "int16" | "int32" | "int64"
                | "uint8" | "uint16" | "uint32" | "uint64"
                | "float" | "float32" | "float64"
                | "bool" | "string" | "null" | "dynamic" | "api_key" ;

Attributes      = { "#" "[" AttrArg { "," AttrArg } "]" } ;
AttrArg         = Ident "=" AttrValue | AttrValue ;
AttrValue       = StringLiteral | IntLiteral | FloatLiteral
                | BoolLiteral | Ident | "[" AttrValue { "," AttrValue } "]" ;

LValue          = Ident | LValue "." Ident | LValue "[" Expr "]" | "*" LValue ;
Path            = Ident { "::" Ident } ;
```

## 5. Core Types

| Type | Description |
|------|-------------|
| `AiResponse<T>` | AI call result with 4 variants: `Success`, `Degraded`, `Refused`, `Error` |
| `Context` | Session context handle (budget tracking, causal DAG) |
| `Model` | Model code (int alias, resolved via `AIAL_MODEL_N`) |
| `api_key` | Opaque type. Cannot be printed, serialized, or leaked. |
| `Usage` | Token usage statistics |

## 6. Built-in Functions

### context module
```
context::new(system_prompt, token_budget, strategy, window_size) → Context
context::current() → Context
context::budget(ctx) → int
context::forget(ctx, msg_id) → void
context::reflect(ctx) → string
```

### privacy module
```
privacy::sensitive(value) → value  (* marks as tainted *)
```

### println
```
println(value)  (* value must not be api_key *)
```

## 7. `#[tool]` Attribute

```aal
#[tool(name = "tool_name", description = "what this tool does")]
fn my_tool(param: Type) -> ReturnType { ... }
```

Registered tools are available to `ask` via the tool dispatch runtime.

## 8. `aial.toml` Config Format

```toml
[capabilities]
allow_network = [
    { provider = "deepseek", models = ["deepseek-v4-flash", "deepseek-v4-pro"] },
    { provider = "openai", models = ["gpt-4o"] },
]

[lints]
unused_match_variable = "warn"
silent_error_discard = "warn"
```

## 9. Environment Variables

| Variable | Purpose |
|----------|---------|
| `AIAL_MOCK=1` | Mock mode (no API key needed) |
| `AIAL_KEY_<PROVIDER>` | Inject API key per provider |
| `AIAL_MODEL_<CODE>` | Override model mapping (e.g. `AIAL_MODEL_0=deepseek:deepseek-v4-flash`) |
| `AIAL_API_URL` | Override API endpoint |

## 10. CLI Reference

```bash
aial <file.aal>                              # Compile & run
aial run <file.aal>                          # Same, explicit
aial check <file.aal>                        # Syntax check only
aial key add --provider <name> --key <key>   # Store API key
aial key list                                # List keys (masked)
aial key remove --provider <name>            # Remove key
aial --philosophy tao <file.aal>             # Tao diagnostic style
aial --philosophy legalist <file.aal>        # Legalist style
aial --philosophy medical <file.aal>         # Medical style
```

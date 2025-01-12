<!-- Generated file. Do not edit. -->

# Language

— Stitch language and compiler contract

### A.1 Lexing

Identifiers are at most 64 ASCII bytes (E0005 beyond). ASCII names: types `[A-Z][A-Za-z0-9_]*`, other identifiers `[a-z_][A-Za-z0-9_]*`; `__` prefix reserved to compiler. Keywords: `fn let var if else while for in return break continue true false nil type migrate spawn yield old and or not`. Integers decimal or `0x[0-9A-Fa-f]+`; floats `[0-9]+\.[0-9]+`, no exponent literal. Longest-match `..` before `.` makes `1..3` unambiguous. Unary-minus parsing accepts magnitude 2^63 only as the direct operand of minus to represent i64::MIN; larger literals E0003. Strings UTF-8, escapes `\n \t \" \\` only; raw newlines forbidden. `#` comment to end of line. `@name` only follows yield. Source/nesting caps per B.8.

### A.2 Grammar

```
program := { typeDecl | fnDecl | globalDecl | migrateType | migrateFn } ;
typeDecl := "type" TYPENAME "{" [ IDENT { "," IDENT } [","] ] "}" ;
fnDecl := "fn" IDENT "(" [ IDENT { "," IDENT } ] ")" block ;
globalDecl := "let" IDENT "=" expr ";" ;
migrateType := "migrate" TYPENAME block ;
migrateFn := "migrate" "fn" IDENT block ;
block := "{" { stmt } "}" ;
stmt := ("let" | "var") IDENT "=" expr ";"
      | lvalue "=" expr ";"
      | "if" expr block { "else" "if" expr block } [ "else" block ]
      | "while" expr block | "for" IDENT "in" expr [ ".." expr ] block
      | "return" [expr] ";" | "break" ";" | "continue" ";"
      | "yield" [ "@" IDENT ] ";" | expr ";" ;
lvalue := IDENT | postfix "." IDENT | postfix "[" expr "]" ;
expr := orExpr ;
orExpr := andExpr { "or" andExpr } ;
andExpr := notExpr { "and" notExpr } ;
notExpr := "not" notExpr | cmpExpr ;
cmpExpr := addExpr [ ("=="|"!="|"<"|"<="|">"|">=") addExpr ] ;
addExpr := mulExpr { ("+"|"-") mulExpr } ;
mulExpr := unary { ("*"|"/"|"%") unary } ;
unary := "-" unary | postfix ;
postfix := primary { "(" [expr {"," expr}] ")" | "." IDENT | "[" expr "]" } ;
primary := INT | FLOAT | STRING | "true" | "false" | "nil" | "old"
         | IDENT | "(" expr ")" | "[" [expr {"," expr} [","]] "]"
         | "{" [STRING ":" expr {"," STRING ":" expr} [","]] "}"
         | TYPENAME "{" [IDENT ":" expr {"," IDENT ":" expr} [","]] "}"
         | "fn" "(" [IDENT {"," IDENT}] ")" block | "spawn" postfix ;
```

Parse an expression statement first, then validate its AST as an lvalue if `=` follows; the grammar does not license assignment to a call result. A `{` in expression position is a map. In condition/for headers, an uppercase name followed by `{` is a record literal; parenthesize it where a following block would otherwise be ambiguous. No chained comparisons/assignment expressions. `spawn f` evaluates f; `spawn f()` evaluates the call first and spawns its result only if that result is a zero-arity closure.

### A.3 Evaluation, scope and numeric rules

Operands, arguments, container elements and record **source fields** evaluate left to right exactly once. Record field layout reordering happens after evaluation through temporary slots. Assignments evaluate target base/index before RHS; setters leave no operand result. `let` binding is immutable but its record/list/map may mutate; `var` allows rebinding. Inner shadowing is allowed, same-block redeclaration E0102. Capturing an immutable binding does not grant mutation. Closures capture shared Box handles, including captures relayed through another closure.

Toplevel declarations allocate stable global symbol IDs before function compilation, allowing mutual recursion. Global `let` initializers execute in source order. A read of a not-yet-initialized global, including through a called function, is E0110 (compile when provable, initialization error otherwise); no silent Nil. Builtin names cannot be redeclared. Exactly one zero-arity `main` is required for run, not parse-only check. Function fallthrough returns Nil. Globals removed by updates become tombstones read as Nil by retained old code; IDs are never reused for a different name.

Only false/Nil are falsy; and/or return the deciding operand. Nil equals Nil; booleans by value; numbers by exact numeric comparison across int/float; strings by UTF-8 content; other heap objects by handle identity. For mixed numeric comparison, compare integer sign/magnitude with decomposed binary64 exponent/significand: never cast a >2^53 int to float and declare a false equality. Arithmetic mixed operands converts to float; this intentional rounding differs from comparison. Int arithmetic, negation, abs and MIN/-1 overflow E0202. Int division truncates toward zero; remainder has dividend sign. Zero divisor E0201 for int/float. Float `%` uses truncated quotient semantics. Nonfinite literal/result/domain failure E0216; normalize -0 to +0. Numeric strings require complete decimal parsing, no whitespace, with optional leading sign; int from float truncates only inside i64 range. NaN/infinity never become guest values or JSON nulls.

Map keys must be strings, missing lookup Nil; lists have checked nonnegative int indices. String ordering bytewise. Record equality is nominal object identity, not structural. Range `a..b` requires ints, snapshots both bounds once, excludes b, empty when a≥b; increment beyond range end never overflows. List iteration stores list handle/index and reads current length each step; mutations can extend/shorten it. Loop variable gets a fresh immutable binding per iteration, including a fresh Box if captured.

### A.4 Formatting and containers

`str`: nil/bools lowercase; ints decimal; floats Rust shortest round-trip with `.0` appended if neither decimal point nor exponent present; strings raw at top level and escaped/quoted in containers; records `Type{field: value, ...}` in layout order; lists `[a, b]`; maps `{"key": value, ...}` sorted by UTF-8 bytes; closures `<fn name>` (`<fn anonymous>`); fibers `<fiber N>`. At depth 8 expand no further and emit `...`, including cycles. Output ≤65,536 bytes; exceeding traps E0206, never truncates silently. No locale formatting.

List capacity ≤65,536; map entries ≤65,536. Maps use a sorted vector of `(String ObjRef, Value)` pairs with binary lookup; insert/erase may move pairs through a continuation. No Rust HashMap embedded in a Buffer. List Buffer slots beyond len are Nil (pop/remove clear them); scanner visits live slots only. Growth allocates/copies before atomically swapping the buffer. Read/write unknown record field E0205. String lengths and substring endpoints are bytes and must be UTF-8 boundaries.

### A.5 Builtins (IDs frozen by order below, starting 0)

| Builtins | Arity and behavior |
|---|---|
| print, str, len | print 0..255, joins str values with one space and newline, returns Nil; str 1; len 1 on string/list/map returns int. |
| push, pop, insert, remove, slice, sort | push(list,v)→Nil; pop(list)→removed; insert(list,i,v) allows i=len→Nil; remove(list,i)→removed; slice(list,a,b) half-open shallow copy; sort(list) mutates stable ascending and returns Nil; only all numbers or all strings, mixed numeric ints/floats allowed. |
| keys, has, delete | keys(map)→sorted string list; has(map,string)→bool; delete(map,string)→bool existed. |
| substr, int, float | substr(s,a,b) half-open; int/float conversions from number or valid numeric string only, invalid E0216. |
| abs, floor, sqrt, sin, cos, min, max | unary except min/max 2; floor returns float; abs preserves numeric kind; sqrt domain x≥0; min/max return the selected original operand, first on tie. |
| rand, rand_int, type_of, frame, heap_stats | rand 0; rand_int(n) n positive int; type_of 1 yields scalar names or nominal record type; frame/heap_stats 0. frame starts at 0 and counts calls to frame, not guest ticks. heap_stats is P5. |
| clear, rect, text, canvas_w, canvas_h | clear(rgb); rect(x,y,w,h,rgb); text(x,y,string); dimensions 0 args. RGB int 0..0xffffff; finite coords; negative width/height E0206. Size from host, native 800×600. All drawing returns Nil. |

Wrong arity statically known E0109; otherwise E0204; invalid kinds use E0200/E0207/E0208 as appropriate. Builtins are compiler-resolved names, not first-class values. P2 implements print/str through CallBuiltin; P3 implements the remainder except heap_stats.

PRNG: xorshift64* state seed; zero seed maps to 0x9e3779b97f4a7c15. Advance x ^= x>>12; x ^= x<<25; x ^= x>>27 (u64 wrapping), output x*2685821657736338717. rand uses top53 bits /2^53. rand_int uses rejection sampling on u64 to remove modulo bias; every rejected sample is charged and can resume. RNG state belongs to Runtime and rolls back on failed update transactions.

### A.6 Bytecode stack convention

The closed Op list is in A.7. PCs are **instruction indices** in Vec<Op>, not serialized byte offsets. Jumps relative to the following instruction. Constants/field names/functions/shape relocations are Code-local pools linked to stable runtime IDs at installation. Old Code always resolves its own pool; it never indexes a newer module's strings.

Call stack before Call: callee, args in order; pop them, reserve locals initialized Nil, parameters occupy first slots, push returned value to caller. StoreLocal/StoreBox/StoreUpval/StoreGlobal pop one; LoadBox/LoadUpval read content, LoadLocal on a boxed slot yields the Box handle for capture construction. MakeBox replaces a slot's value with a Box and preserves the original value. Closure pops exactly ncaptures Box handles in declared capture order. GetField pops object/pushes value; SetField consumes object,value; Index consumes object,index; SetIndex consumes object,index,value. NewRecord consumes field values in layout order; NewMap consumes key,value pairs; literals leave one object. Return consumes one value, with implicit Nil emitted if absent. JumpIfFalse consumes condition; Peek variants retain it; compiler emits Pop on the evaluated-right branch for short-circuit operators. Yield consumes nothing; Spawn consumes closure/pushes fiber.

IterInit(slot,0) consumes list and writes list/index into two hidden locals; kind1 consumes **start then end** from stack (end popped first) and writes current/end. IterNext either pushes next value and advances or jumps with no push. Compiler clears hidden locals when loop exits. Hidden locals are keyed by enclosing yield label plus loop kind and loop variable for migration, not emission ordinal. A loop containing several labels records the same loop identity in each descriptor. Changing enclosing loop topology makes C.4 reject.

### A.7 Closed instructions and verifier

```
Const(u32) Nil True False Pop Dup
LoadLocal(u16) StoreLocal(u16) MakeBox(u16) LoadBox(u16) StoreBox(u16)
LoadUpval(u16) StoreUpval(u16) LoadGlobal(u32) StoreGlobal(u32)
GetField(u32) SetField(u32) Index SetIndex
NewRecord(u32) NewList(u32) NewMap(u32) Closure(u32,u16)
Call(u8) Return Jump(i32) JumpIfFalse(i32) JumpIfFalsePeek(i32) JumpIfTruePeek(i32)
IterInit(u16,u8) IterNext(u16,i32)
Add Sub Mul Div Mod Neg Eq Ne Lt Le Gt Ge Not Yield(u32) Spawn CallBuiltin(u16,u8)
```

Label sentinel u32::MAX means unlabeled. Collection literal operands are u32 so the legal 65,536 boundary is representable. P2 implements scalar/control/closure/range/CallBuiltin print+str; P3 containers and other builtins; P4 Yield/Spawn. No partial opcode decoder or serialized external bytecode input is shipped.

Verifier runs after compilation/linking: operand/pool bounds, arity/capture count, jump target, control-flow stack heights equal at joins, no underflow, max locals/stack/frames, Return shape, expression stack empty at **every** Yield. IterNext has separate exhausted/success stack effects. Invalid internal bytecode E0112, not unsafe execution. Unit-test deliberately malformed modules.

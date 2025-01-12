<!-- Generated file. Do not edit. -->

# Runtime

— Heap, scheduling and resource specification

### B.1 Handles, layouts and ownership

PAGE_BYTES=262144, LARGE_OBJECT_BYTES=131072 including header/alignment. Payload header **16 bytes from a later owner onward**: size u32, kind u8, flags u8, reserved u16=0, back_slot u32, back_generation u32. Every size rounds up to 16 with checked arithmetic. ObjRef is slot+generation; reuse increments generation; wrap retires the slot. Slots store address `(page:u32, offset:u32)`, size, kind, shape, epoch/color; free-list linkage is separate from an address.

Arena Value representation is exactly 16 bytes: tag u8 (Nil0/Bool1/Int2/Float3/Obj4), seven zero bytes, eight-byte little-endian payload. Zero reserved bits required; Bool only 0/1; Obj packs generation/slot. No reliance on `size_of::<Value>()`, enum layout or pointer width. Accessors decode to owned Value copies and never lend memory across operations.

Bodies after header: String `(len:u32, UTF8 bytes)`; Buffer `(len:u32, capacity:u32, encoded Values)`; List `(len:u32, cap:u32, buffer:ObjRef, mutation_epoch:u64)`; Map same with interleaved key/value Values and pair count; Record shape ID + fields; Closure code handle + capture count + Box handles; Box one Value; Code side-table ID + explicit reference array; Fiber side-table ID; Husk old shape + old payload descriptor owned by a transaction. No two independently swept slots own the same allocation. Transfer old payload ownership to a Husk changes its header back-reference; rollback restores it. Shape IDs never reused within Runtime.

Heap writes centralize checks, epoch and barrier before storing. VM-owned external roots (globals/stacks/frames/pins/transaction temporaries/continuations) use root-store helpers. Debug root audit independently walks all root families and compares with registration. Side tables and descriptors are included in resource accounting.

### B.2 Allocation and pages

Normal pages use bump allocation and states Allocating/Full/Evacuating/Free. At most one mutator and one evacuation destination page are partially allocating. Whole dead pages return to Free at sweep; dead holes in other pages await compaction. Reuse lowest-index free page before growing. Large payloads occupy ceil(size/PAGE_BYTES) dedicated units and never move. All units count against max-pages, including Free retained reserve; separately report occupied/retained/free. `bytes_live` is allocated payload bytes of unswept objects, not an omniscient reachable-byte count.

On allocation failure do not run GC recursively in an opcode. An allocating continuation roots its operands, requests collection and parks; service scheduling may reclaim memory. Retry once after a complete collection/compaction opportunity; if no adequate space, E0211 with unchanged target object. Migration/update staging uses its explicit reserve and fails atomically instead of parking. Empty large pages may be released; retained free normal reserve is at most 2 after a quiescent drain. Heap-map represents each256 KiB unit, Free255, occupied0..100; large objects fill each covered unit proportionally.

### B.3 Incremental collector

Use epoch marking and a Dijkstra insertion barrier with states Idle/Roots/Mark/Sweep. Starting a cycle captures the current slot high-water mark and a cursor over registered roots. Visit ≤64 root/value edges per microstep, not entire stacks at once. Root writes while a cycle is active shade the new target immediately. New allocations during an active cycle are current-epoch black; initialization stores still barrier their children. Every heap reference store during an active cycle shades a white target before publication, including stores into gray partially scanned objects. This conservative insertion barrier also shades references written into white objects; that may retain floating garbage for one cycle but cannot lose a live child.

Gray objects carry `(ObjRef, child_cursor)`; trace at most 64 Values/references per step, including large buffers and Code reference arrays. Blacken only when enumeration finishes. Mutation of a region already scanned invokes the same barrier. Fiber stacks/frame handles/continuation roots are scanned as registered root chunks, not an unbounded final rescan. When initial root cursor is complete and gray queue empty at the service point, begin Sweep with a retained slot cursor; no mutator instruction runs between the empty check and transition.

Sweep at most 64 captured slots per step. A slot not marked in this epoch is freed; never reset survivors to white during sweep. Active-cycle barriers remain enabled through Sweep; newly gray work is drained before continuing the sweep cursor. New slots beyond captured high-water survive this cycle. Avoid global “paint every slot white” scans; epoch increment replaces them. Epoch wrap invokes a fully budgeted reset state before starting another cycle. Disposal of Code/Fiber side tables is cursor-based and charged.

Code constants/nested Code objects/captures are traced; finished fibers clear their stacks and leave the scheduler root list but remain identity values if guest code retains them. Empty/self-referential cycles without roots are collectible. No finalizer or weak reference is shipped.

### B.4 Incremental compaction

Only normal pages; pause the compactor when GC has active work. Trigger when dead bytes in Full pages exceed 25% of normal occupied bytes or normal occupied pages exceed 1.5×ceil(normal_live_bytes/PAGE_BYTES)+2. Selection scans one page per microstep and selects every Full page with live fraction≤2/3, ordered by fraction then page index. Enumerate their live payloads incrementally into a max-priority queue ordered by size descending then ObjRef; queue operations are charged and bounded by the slot cap. Evacuate in that order into the single destination bump page, validating generation/address before every move. In a quiescent drain, descending sizes≤half a page ensure every sealed destination except the final partial page is at least 2/3 full; this is the packing basis for the 1.5× bound. A fixed<50% victim rule cannot establish that bound and leaves 51%-live pages stranded. Header enumeration is cursor-based; generation/address checks prevent a reused slot from reviving stale bytes. GC/mutator changes invalidate stale queue entries and schedule another bounded selection pass; quiescent drain ends only after rechecking the bound. Copy headroom is an admission invariant, not a last-minute allocation hope: outside evacuation, permit at most floor((max_pages-large_page_units)/2) occupied normal pages. The remaining capacity can hold a full replacement of those pages; it need not be physically allocated until copying. Free retained pages count toward physical reservation and can supply that capacity. Thus max-pages is a total arena cap, not a promise of that many pages of guest payload. During one evacuation epoch, park allocating continuations/new migrations, but permit nonallocating mutation and GC; the service continues until its reserved copy work completes. GC may cancel dead payloads. After the epoch, resume allocations under the recalculated invariant. An immediate update requiring allocation while evacuation is active rejects E0309 with compaction-busy cause before any change; a queued safe-point update waits for compaction to finish within its existing deadline. This prevents a size-sorted copy from consuming one spare page before any source page is completely empty. Reject max-pages<2 before execution.

Evacuate into a different page. Copy ≤4096 bytes per microstep into reserved destination while Slot still points to source. Mutator writes during an incomplete copy mirror that value write into the already-copied destination range; stores to uncopied regions will be copied later. Record the moving object's source/destination/cursor in the heap, protect its current source/destination reservation from reclamation, and prevent a second move or migration of it until copy commits. This protection is not a reachability root; GC can still prove the logical object dead and cancel the move. GC may pause/resume a move; if the object dies, cancel destination and reclaim both correctly. Commit Slot address atomically after final chunk, update page/live counters once. Never scan/build an entire page's object vector synchronously.

An object waiting on an active move resumes its lazy migration after move completion; scheduler continues other work. This interaction has a dedicated test. Drain quiescent work in tests until no eligible page remains, then apply the normal-page bound; do not count internal large-object slack as a fragmentation defect.

### B.5 Work units, scheduling and continuations

One work unit: one ordinary opcode, one examined Value/reference, one scalar sort comparison/move, one copied16-byte block, or one table/page metadata item. A microstep performs at most 64 such operations, or a copy of at most 4096 bytes (=256 units); declared costs are charged **before** executing and split to fit remaining Units budget. Host output copies charge bytes/16 rounded up. Migration transactions are the explicitly bounded atomic exception in C.3, charged by their full fuel/reservation.

Units K: K=0 does no guest/service work; reserve ceil(K/4) for services when pending, remaining for mutator. Mutator slice is min(1000, remaining mutator units), with rotating runnable cursor carried across frames. Spawned fibers join next frame; yielded fibers wake next frame; preempted fibers rejoin this frame at tail. Completed fibers leave the run queue immediately. Service selection rotates among active GC/compaction/migration while respecting GC/compaction exclusion; each active eligible service gets a step before one receives a second. Unused reservations may be borrowed. Never invent an extra10% after the deadline.

Micros U: one absolute deadline start+U, no calibration loop. Check host clock before/after each microstep; use the same fixed maximum work sizes, stop starting work at deadline. Aim 75% mutator then services; services get remaining actual time. A clock moving backward E0212; a frozen synthetic clock remains bounded by a secondary cap of 1,000,000 work units per frame. Measure maximum step and actual overrun. OS interruptions between clock reads are retained in measurements. Administrative scans/report snapshots are incrementally maintained; export/JSON copying and browser painting after frame return are measured separately in the host envelope; bounded Host callbacks during frame are included in runtime/admin time.

Large string operations, equality/order scans, map search/shift, list copying/insertion/removal/sort, allocation initialization, deep str and PRNG rejection use interpreter `PendingBuiltin` continuations from P4. Root their operands/temporary buffers; resume before advancing the suspended opcode PC. For a multi-step mutating container builtin, copy into private staged storage, remember source mutation_epoch, then check epoch and swap once complete. If another fiber changed the container, discard and trap E0214; no partial reorder is published. `sort` is bottom-up stable mergesort. Parsing/compilation/update preparation are explicit user operations outside animation frame budgeting and their pauses are separately shown.

### B.6 Determinism

Same source, seed, Units budget, update schedule and toolchain/lockfiles ⇒ identical stdout, canonical reports and draw bytes across native macOS/Linux/wasm. `now_us` is never called in Units mode, including construction/updates. `_us` fields are zero. All tie-breaks use stable numeric IDs/UTF-8 names. Float results are finite, -0 normalized and portable libm paths used. Tests include >2^53 comparisons, signed zero, sqrt/trig and draw encoding. No wall-clock metadata in canonical report payload; provenance goes into a separate envelope.

### B.7 Reports and accounting

Counters have explicit interval or lifetime meanings in E. Mutator time includes lazy migration; lazy migration time is a **subset**, never added twice in stacked totals. Background migration is a service category. Header/payload live counts include Husks while a transaction is active; public reports taken at frame boundary cannot expose an uncommitted transaction. Fragmentation denominator excludes large pages and handles zero normal bytes as0. Heap totals reconcile pages and object categories; report counters use checked arithmetic.

### B.8 Resource caps

| Resource | Cap / failure |
|---|---|
| source / parser nesting | 1 MiB /256; E0008 |
| frames per fiber / locals per function | 1024 /4096; E0210 /E0112 |
| values per fiber / fibers | 65,536 /1024; E0210 |
| list/map entries / record fields | 65,536 /64; E0206 /E0104 |
| string bytes / str output | 16 MiB /64 KiB; E0206 |
| page units / metadata bytes | configurable --max-pages default 4096 with B.4 copy headroom; VM metadata cap 128 MiB; E0211 |
| object slots / registered pins | 1,048,576 /4096; E0211 |
| source declarations / code instructions per module | 65,536 /1,048,576; E0112 |
| output per frame | 16,384 draw commands and 1 MiB text/stdout bytes; E0212 before enqueue beyond cap |
| migration / update staging | C.2–3; E0309 or E0213 |

Caps include pending state, not just committed state. Per-runtime metadata uses checked reservations; process RSS can exceed arena bytes, so neither the page cap nor bytes_arena is advertised as a total-process-memory bound.

### B.9 Independent heap model

Generate Alloc/Link/Unlink/AddRoot/DropRoot/GcStep/CompactStep/FullDrain operations over records, buffers, closures, code and cycles. Use a separate graph model keyed by ModelId, never the collector's edge enumerator. After each operation every model-reachable value remains readable and equal, handle generations reject stale references, and insertion invariants hold. Incremental marking can retain floating garbage: **FullDrain** stops mutations and completes two full cycles, then compaction to quiescence; only then require exact reachable/live-set equality. Include mutator stores during tracing/sweep/copy and pending-object roots. 2,000 ordinary cases; same strategy100,000 in ignored `heap_model_long`. Persist minimized failures in proptest-regressions.

— Updates and migration

### C.1 Version and descriptor lifetime

Full-source updates preserve existing global values; initializer changes to an existing `let` do not reset it. New globals append; removed IDs tombstone; reintroduced same name reuses its own ID with a new value. New calls through globals use newest Closure; saved closures retain their Code. Code comparison ignores spans/comments but includes instructions, literals, capture layout and label descriptors; a fully identical update returns `changed:false` without revision increment.

Type shape version increments only when ordered field names change; program revision increments once per non-noop successful update. Code-local name/shape pools remain stable. Old shapes and recipes remain while **instances, Code allocation sites, frame descriptors or another retained recipe** can require them. Reclaim metadata by this dependency graph, not instance count alone. Slots/IDs are never reassigned to another nominal type. Remove a type only if none of those live references exists, E0302 otherwise.

### C.2 Preparation, validation and atomic apply

Prepare parses/compiles against immutable registry descriptors and returns all diagnostics sorted by source/code. Validation order: syntax/compile E0300; type changes/recipes E0301/2/4/5; frame label/scope/layout compatibility E0303/6; effect checks E0309. E0300 retains nested E00/E01 codes. Every migration body is checked even if zero instances currently exist.

Apply verifies runtime owner/base revision, not inside frame, and unfinished initial bootstrap is false. New code/globals, transformed frames and registry delta are staged in a **transaction arena** with its own allocation journal. Reads of existing objects allowed; writes to existing heap objects/globals, host calls, RNG, spawn/yield, and calls to arbitrary guest functions are forbidden in update initializers/frame recipes. The exact read-only builtin allowlist is str, len, slice, keys, has, substr, int, float, abs, floor, sqrt, sin, cos, min, max and type_of. push, pop, insert, remove, sort and delete are allowed only on transaction-owned containers. Constructors are allowed. print, rand, rand_int, frame, heap_stats, clear, rect, text, canvas_w and canvas_h are forbidden, as is spawn. The compiler checks A.5's builtin IDs, not name-substring matching; all loops/scans/copies consume the same transaction fuel. Static checking rejects obviously forbidden effects; runtime ownership guard catches indirect aliases. Existing globals are accessible read-only. Total update fuel 1,000,000 units and new staged payload 16 MiB; exhaustion/trap/allocation failure E0309 with inner cause. No executable initializer work occurs during prepare.

Create all potentially fallible allocations/validations before publication. Commit registry/global/frame pointers and shared-Box assignments as a no-fail journaled batch; no guest callback during commit. On any precommit failure discard staged objects and restore free-list/generation/counters/RNG/root accounting exactly. No staged handle escapes to a callback/report. UpdateReport storage and all publication-journal capacity are reserved before commit; constructing the success result must not introduce a fallible postcommit allocation. UpdateReport is returned only after commit; failed apply returns errors and measured preparation/application duration in the host error envelope. Byte-for-byte semantic snapshot tests include heap graph, old frames, registry, RNG and accounting.

### C.3 Object migration

Each changed type needs exactly one `migrate T` for the immediately previous shape; unknown/unchanged type E0304, unknown target field E0305. The block has `old` read-only and implicit new-field assignment targets. Lexical body locals take precedence; otherwise bare assignment names target fields, bare reads resolve body locals then globals. Use `old.f` for old fields, including removed fields. `old` cannot be stored or returned, passed into a closure, or assigned to a target field; use its field values instead. This prevents Husks escaping their transaction.

A migration access begins a bounded transaction overlay keyed by original ObjRef. Reserve a new Nil-filled record, retain old payload as transaction-owned Husk, expose staged new record through the overlay, then execute recipe. Self/nested cycle reads see the new shape with unset fields Nil. Reads of another old record may stage its migration in the same overlay; all nested steps share fuel and commit together. Every successful chain executes v1→v2→v3 in order. Outside the transaction the Slot continues to point to the valid old object until commit.

Recipes use the exact builtin allowlist in C.2, can read old graphs, construct/copy records/lists/maps, loop, and mutate **only** their transaction-owned allocations/target records. No arbitrary guest call, host draw/print, RNG, spawn, yield, return, closure creation, global rebinding, or mutation of pre-existing lists/maps/boxes. Compile effect validation and dynamic ownership checks both apply. Atomic transaction caps: **2048 accounted units, 32 KiB staged payload, 64 nested records/call frames**. Count every loop iteration/scan/copy/allocation; a loop without yield cannot run forever. The scheduler starts an atomic transaction only when at least 2048 units remain; in Micros mode it uses its measured cost estimate and records full elapsed time. Tiny Units budgets park that access with an explicit `needs_units=2048` report; for any blocked atomic output command, needs_units reports its smallest required charged cost, and it is 0 when no atomic work is blocked; they never silently overspend or spin internally.

On success publish all staged payload pointers/shape counts atomically, release Husks exactly once and clear transaction roots. Missing **assigned** target fields emit W0400 once per type/shape; explicit Nil is not “missing”. On fuel/OOM/guest error discard the overlay, leave original objects unchanged, memoize a failed `(object generation,target shape)` attempt and trap E0213 with inner cause. Further access returns the same failure; background scanning skips that failed pair and reports it once. New successful updates that supply a new target can attempt a new chain; a known failing required earlier recipe still fails deterministically. Reset/new Runtime is the recovery from a permanently invalid recipe; no partially initialized object masquerades as current.

Background cursor examines a bounded number of slots, schedules one transaction at a time on a rooted internal stack, rotates with other services and wraps until no eligible instances remain. It does not require a runnable guest fiber. Old Code creating old-shape instances marks migration work pending again. Lazy migration waits for an in-progress compaction move of the same object; no simultaneous payload ownership transitions.

### C.4 Suspended frames

Only the top frame suspended at **named** Yield in a changed or removed top-level function f is eligible for validation; removal of such an active function rejects E0303. Anonymous closure frames and lower frames keep old code and receive W0401. Missing f/label rejects E0303; unlabeled/preempted frames continue old code. A descriptor contains resume PC **after** Yield, live source-local names/binding kind/boxed state, lexical-scope path and enclosing loop `(kind,loop variable,hidden state names)` list. Named Yield must have unique live names; shadowed duplicates at that point are E0106. Compare old/new descriptors, not compiler ordinal slots.

Transfer same-named live locals; preserve the identical Box if boxed both sides. Boxed→unboxed or unboxed→boxed transfer is rejected E0303 where an existing closure could observe a split variable; no warning silently permits broken sharing. New locals default Nil with W0400 unless assigned by `migrate fn f`. Removed locals disappear after commit. Compatible enclosing loops preserve hidden list/index or range current/end. Removing/reordering/changing an enclosing loop rejects E0303; inserting a separate completed loop before the label does not rename the retained loop. No “restart loop” guess.

A frame recipe is compiled separately for each affected label descriptor. Reads: body locals, then immutable old-local environment, then globals/builtins. Writes: body locals or **new-local environment** only. Thus `x=x+1` reads old x and assigns new x; a subsequent `x` still reads old x unless a body-local shadows it. All referenced names must exist at every affected descriptor, else E0305. Global named function calls remain forbidden by the effect boundary. Box content assignments are staged until the same whole-update commit; conflicting assignments to one shared Box from different frame recipes reject E0309. No frame changes if another frame's recipe fails. Migrated frames retain fiber identity and scheduler position.

— Reports, fixtures and CLI

### E.1 Report schema 1

Serialize in declared field order; all counters integer, all times microseconds. JSON u64 counters beyond 2^53-1 use decimal strings consistently in native/web; schema tests cover this boundary. Web converts displayed counters explicitly, not with lossy Number coercion. Runtime internal reports are Rust structs, serde adapters live only at hosts.

FrameReport fields: `schema=1, frame, version, budget_kind, budget, work_units, mutator_units, gc_units, compact_units, migrate_bg_units, admin_units, mutator_us, gc_mark_us, gc_sweep_us, gc_root_us, compact_us, migrate_bg_us, migrate_lazy_us, admin_us, total_runtime_us, overrun_us, max_step_us, needs_units, gc_state, bytes_live, bytes_arena, metadata_bytes, normal_live_bytes, normal_pages_occupied, large_page_units, pages_reserved, pages_free, frag_ratio, objects, objects_moved, bytes_moved, objects_migrated, objects_moved_total, objects_migrated_total, fibers_total, fibers_runnable, shapes_live, shapes_as_of_frame, traps, traps_pending, traps_total, service_errors, service_errors_pending, service_errors_total, warnings, warnings_pending, warnings_total`.

Interval counters reset each frame; `_total` lifetime. `shapes_live` sorted type/version with instance and retained-code-reference counts. `traps={fiber,code,message,trace}`; service errors omit fiber; `warnings={code,message}`. `work_units=mutator_units+gc_units+compact_units+migrate_bg_units+admin_units`; lazy work included in mutator_units. `total_runtime_us` is direct end-start measurement including report assembly, and category sum must not exceed it; `migrate_lazy_us` is a subset of mutator_us. Host envelope adds serialize/render/frame-start lateness and update pauses separately. Per-frame `shapes_live` is an immutable reference-counted cached snapshot built by administrative steps of at most 32 rows, with `shapes_as_of_frame`; report return clones only that handle. Tests requiring current per-shape counts run a quiescent drain first. Report event lists contain at most 32 items each; pending counts and total failures remain visible and `--assert` uses totals, so a delayed event cannot disappear. Traces contain at most 32 frames plus a truncation count, messages at most 512 bytes. Event preparation is charged incrementally; no unbounded registry/stack traversal is hidden in report construction. `_us=0` in Units.

UpdateReport: `schema,changed,from_version,to_version,functions_replaced,globals_added,globals_removed,types_changed,shapes_added,frames_migrated,frames_in_old_code,work_units,init_us,frame_migrate_us,warnings`. Units timings0. Preparing a no-op still validates full source; scheduled no-op counts as an applied update with changed=false.

Soak JSON is a streaming run envelope: provenance + requested updates + observed update outcomes + exact totals/maxima + logarithmic timing histogram (1 us buckets through 16,384 us, overflow bucket) + summary, with an optional separate JSONL per-frame stream. Do not hold two hours of full per-frame/shape arrays in memory. Percentiles use the nearest-rank histogram upper bound and state the 1 us resolution; every sample counted. A failure/missing or duplicate update makes `--assert` exit5. Final memory checks occur after a quiescent service drain whose frames are identified separately, not included in active-frame timing percentiles.

### E.2 Update fixture schema 1

```
{
  "schema":1, "seed":1, "budget_units":100000, "frames":100,
  "updates":[{"frame":10,"file":"v2.stitch","expect":"applied","codes":[]}],
  "stdout":["expected line"],
  "checks":[{"source":"frames","select":"last","field":"objects","op":"le","value":100}],
  "update_checks":[{"index":0,"field":"frames_migrated","op":"eq","value":1}]
}
```

`expect` applied/rejected; codes sorted exact. Checks source frames/summary, select last/max/sum, scalar field whitelist, op eq/le/ge; all specified checks required; unsupported/unknown fields reject fixture before running. Shape checks use separate `{type,version,instances}` selectors. No vague min/max over unspecified intervals. Frames0..N-1, update before that frame, distinct ascending frames <N, filename exists, budget/frames positive; scenario harness continues after expected rejection. stdout lines exclude trailing newline. Frame-migration counts are per update, object migration may happen many frames later: use sum/summary totals, not the last frame's interval count.

### E.3 CLI surface

`check FILE`; `dump FILE --ast|--bytecode`; `test DIR`; `run FILE`; `soak FILE`.

Run/soak flags: `--seed U64` default 1; `--deterministic`; `--frames N`; `--seconds S` (soak only, mutually exclusive with frames); `--budget-us U` default 8000; `--budget-units K` default 1,000,000 when deterministic; `--max-pages P` default 4096; `--update FRAME:FILE` repeatable; `--report PATH`; `--draw-stats`; `--assert` (soak only); verbosity flags. Units implies deterministic; deterministic+budget-us is usage error. Zero budgets are supported by library tests, CLI requires positive budgets; frames0 is allowed and writes empty run evidence but cannot pass soak assertions. Without frames, run finishes when all guest fibers finish; bootstrap is part of fiber0. With frames, run exactly N frame calls even if guest work finished, allowing scheduled update/drain fixtures. Drain policy never calls guest code after requested active frames. Validate all update file/frame arguments before starting; source compilation occurs at its scheduled boundary.

bootstrap declares all types and future boundary stubs; the phase column in contracts.json owns activation. Internal helpers can be added without inventing another public surface.

```rust
// st-heap; fields private, factories check ranges/generation.
pub struct ObjRef(core::num::NonZeroU64); // high 32 generation, low 32 slot; slot 0 invalid
pub struct ShapeId(u32);
pub struct Shape { pub type_name: String, pub version: u32, pub fields: Vec<String> }
pub enum Value { Nil, Bool(bool), Int(i64), Float(f64), Obj(ObjRef) }
// This Rust enum is an API value, never its arena byte representation.

// st-vm; Runtime owns a Box<dyn Host>.
pub enum Budget { Units(u64), Micros(u64) }
pub trait Host {
    fn now_us(&self) -> u64;
    fn print(&mut self, chunk: &str, end_line: bool) -> Result<(), HostError>;
    fn draw(&mut self, batch_id: u64, commands: &[DrawCmd], complete: bool) -> Result<(), HostError>;
    fn canvas_size(&self) -> (i64, i64);
}
impl Runtime {
    pub fn new(source: &str, seed: u64, host: Box<dyn Host>) -> Result<Self, BuildError>;
    pub fn frame(&mut self, budget: Budget) -> FrameReport;
    pub fn prepare_update(&self, source: &str) -> Result<UpdateBundle, UpdateError>;
    pub fn apply_update(&mut self, bundle: UpdateBundle) -> Result<UpdateReport, UpdateError>;
    pub fn heap_map(&self) -> Vec<u8>;
    pub fn queue_update(&mut self, bundle: UpdateBundle, targets: Vec<SafePoint>, max_frames: u32) -> Result<(), UpdateError>;
    pub fn take_update_result(&mut self) -> Option<Result<UpdateReport, UpdateError>>;
    pub fn pin(&mut self, value: Value) -> Result<PinId, VmError>;
    pub fn unpin(&mut self, pin: PinId) -> Result<(), VmError>;
}
```

`Runtime::new` parses/compiles without executing guest code or reading time. Initializers execute as fiber 0 before main under frame budgets. Pin IDs are generational and strong; an unpinned raw ObjRef held by a host is not a root and later access can return E0215. Timing mode follows the most recent frame budget, default Units before the first frame; preparation itself never reads the clock, and hosts measure its external pause. Runtime is neither Send nor Sync. Host callback errors become E0212. No callback can re-enter the active runtime; adapter methods reject E0307 before touching it.

`UpdateBundle` owns a pure compiled module/diff, base revision and an opaque `Rc<()>` owner token. `prepare_update(&self)` allocates no guest heap objects and changes no registry, counter, RNG or roots. Bundle owner is checked by `Rc::ptr_eq`; base revision checked at apply; mismatch E0308. Dropping a bundle needs no heap unpin. `apply_update` consumes it and may fail; C owns rollback.

`SafePoint { function: String, label: String }` selects all currently live fibers executing that named function; each supplied target must select at least one. Queueing captures their fiber IDs, permits only one pending request, and rejects missing targets, zero deadline, or a stale bundle. Later spawns do not join the captured set. If a captured fiber finishes/traps before reaching its point, reject and release all holds. Before each frame's timed region, if every captured fiber is held at its label and compaction is idle, apply the transaction with the frame-active flag clear; expose its outcome through `take_update_result`. Preparation/application timing belongs to the update envelope. A deadline counts calls to frame after queueing; at its expiry return E0303 and release held fibers. The guest cannot enqueue its own updates. P9 implements this surface; P7 immediate apply remains useful.

Output continuations call Host.print with UTF-8-boundary chunks of at most 4096 bytes; end_line is true only on the final chunk (including an empty final line). Host.draw receives at most 64 commands and 4096 canonical encoded bytes per callback, with batch_id stable across chunks and complete true only on its final chunk. Reference hosts buffer these bounded chunks; terminal/file output occurs after frame returns. Charge copy work before each callback, and include callback elapsed time in the charged administrative category. Draw Text is capped at 4000 UTF-8 bytes (E0212) so one command fits a transfer chunk. Browser host buffers chunks until complete; hashing uses commands only and is independent of transfer boundaries.

`DrawCmd = Clear{rgb:u32} | Rect{x:f64,y:f64,w:f64,h:f64,rgb:u32} | Text{x:f64,y:f64,text:String}`. FNV-1a64 draw hash: tag u8 (0/1/2), numeric fields in listed order as little-endian IEEE bits/u32, text u32 byte length then UTF-8. Offset basis 14695981039346656037, prime 1099511628211, wrapping multiplication. Normalize numeric -0 to +0 before encoding. No debug formatting in the hash.

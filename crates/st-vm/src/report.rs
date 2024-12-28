//! Per-frame report. Schema 1 field names; all counters start at zero.

/// Frame counters and event lists. Event lists stay empty until the runtime runs.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FrameReport {
    /// Report schema. Always 1.
    pub schema: u8,
    /// Frame index, starting at 0.
    pub frame: u64,
    /// Guest source version string.
    pub version: String,
    /// `units` or `micros`.
    pub budget_kind: String,
    /// Requested budget.
    pub budget: u64,
    /// Sum of category units.
    pub work_units: u64,
    /// Mutator work, including lazy migration.
    pub mutator_units: u64,
    /// Mark/trace work.
    pub gc_units: u64,
    /// Compaction work.
    pub compact_units: u64,
    /// Background migration work.
    pub migrate_bg_units: u64,
    /// Administrative work.
    pub admin_units: u64,
    /// Mutator microseconds. Zero in Units mode.
    pub mutator_us: u64,
    /// GC mark microseconds.
    pub gc_mark_us: u64,
    /// GC sweep microseconds.
    pub gc_sweep_us: u64,
    /// GC root microseconds.
    pub gc_root_us: u64,
    /// Compaction microseconds.
    pub compact_us: u64,
    /// Background migration microseconds.
    pub migrate_bg_us: u64,
    /// Lazy migration microseconds (subset of mutator).
    pub migrate_lazy_us: u64,
    /// Administrative microseconds.
    pub admin_us: u64,
    /// End-to-end frame microseconds.
    pub total_runtime_us: u64,
    /// Overrun microseconds.
    pub overrun_us: u64,
    /// Longest microstep microseconds.
    pub max_step_us: u64,
    /// Units still needed after the budget.
    pub needs_units: u64,
    /// Collector state name.
    pub gc_state: String,
    /// Live payload bytes.
    pub bytes_live: u64,
    /// Arena bytes.
    pub bytes_arena: u64,
    /// Side-table metadata bytes.
    pub metadata_bytes: u64,
    /// Live bytes on normal pages.
    pub normal_live_bytes: u64,
    /// Occupied normal pages.
    pub normal_pages_occupied: u32,
    /// Large-object page units.
    pub large_page_units: u32,
    /// Reserved free pages.
    pub pages_reserved: u32,
    /// Free pages.
    pub pages_free: u32,
    /// Fragmentation ratio × 1e6, stored as integer.
    pub frag_ratio: u64,
    /// Object count.
    pub objects: u64,
    /// Objects moved this frame.
    pub objects_moved: u64,
    /// Bytes moved this frame.
    pub bytes_moved: u64,
    /// Objects migrated this frame.
    pub objects_migrated: u64,
    /// Lifetime objects moved.
    pub objects_moved_total: u64,
    /// Lifetime objects migrated.
    pub objects_migrated_total: u64,
    /// Fiber count.
    pub fibers_total: u32,
    /// Runnable fibers.
    pub fibers_runnable: u32,
    /// Live shapes.
    pub shapes_live: u32,
    /// Frame whose shape snapshot this is.
    pub shapes_as_of_frame: u64,
    /// Traps this frame (capped).
    pub traps: u32,
    /// Pending trap count.
    pub traps_pending: u32,
    /// Lifetime traps.
    pub traps_total: u32,
    /// Service errors this frame.
    pub service_errors: u32,
    /// Pending service errors.
    pub service_errors_pending: u32,
    /// Lifetime service errors.
    pub service_errors_total: u32,
    /// Warnings this frame.
    pub warnings: u32,
    /// Pending warnings.
    pub warnings_pending: u32,
    /// Lifetime warnings.
    pub warnings_total: u32,
}

impl FrameReport {
    /// Schema-1 zeroed report.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            schema: 1,
            gc_state: "idle".to_owned(),
            budget_kind: "units".to_owned(),
            version: String::new(),
            ..Self::default()
        }
    }
}

/// Outcome of a successful or rejected update. Zeroed until apply exists.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpdateReport {
    /// Report schema. Always 1.
    pub schema: u8,
    /// Whether guest code changed.
    pub changed: bool,
    /// Previous version.
    pub from_version: String,
    /// New version.
    pub to_version: String,
    /// Functions replaced.
    pub functions_replaced: u32,
    /// Globals added.
    pub globals_added: u32,
    /// Globals removed.
    pub globals_removed: u32,
    /// Types changed.
    pub types_changed: u32,
    /// Shapes added.
    pub shapes_added: u32,
    /// Frames migrated.
    pub frames_migrated: u32,
    /// Frames left in old code.
    pub frames_in_old_code: u32,
    /// Work units charged.
    pub work_units: u64,
    /// Initializer microseconds.
    pub init_us: u64,
    /// Frame-migration microseconds.
    pub frame_migrate_us: u64,
    /// Warning count.
    pub warnings: u32,
}

impl UpdateReport {
    /// Schema-1 empty report.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            schema: 1,
            ..Self::default()
        }
    }
}

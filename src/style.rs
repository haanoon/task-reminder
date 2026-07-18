/// Midnight Velvet stylesheet — deep purple-black glassmorphism with
/// lavender accents, warm text, and refined micro-interactions.
pub const CSS: &str = r#"
/* ══════════════════════════════════════════════════════════════════════
   Color palette (conceptual reference — used inline below)
   --bg-deep:      rgba(10, 8, 20, 0.88)    deep purple-black
   --bg-panel:     rgba(18, 14, 32, 0.82)   panel surface
   --bg-elevated:  rgba(28, 22, 48, 0.75)   slightly elevated
   --border:       rgba(160, 130, 220, 0.12) soft violet border
   --border-focus: rgba(180, 155, 235, 0.45) focused border
   --accent:       #b49ee0                   lavender
   --accent-glow:  rgba(180, 158, 224, 0.25) accent glow
   --text-primary: #ede8f8                   near-white with violet tint
   --text-muted:   #8878a8                   muted lavender-gray
   --text-dim:     #5c5478                   dim text
   --success:      #82c8a0                   soft sage green
   --danger:       #d08898                   muted rose
   --warn:         #c8a86e                   warm amber
   ══════════════════════════════════════════════════════════════════════ */

/* ══════════════════════════════════════════════════════════════════════
   Window — fully transparent host
   ══════════════════════════════════════════════════════════════════════ */
window,
window.background {
    background-color: transparent;
}

/* ══════════════════════════════════════════════════════════════════════
   Main Glass Panel
   ══════════════════════════════════════════════════════════════════════ */
.main-container {
    background-color: rgba(10, 8, 20, 0.88);
    border-radius: 20px;
    border: 1px solid rgba(160, 130, 220, 0.12);
    box-shadow:
        0 4px 40px rgba(0, 0, 0, 0.55),
        inset 0 1px 0 rgba(255, 255, 255, 0.04);
}

/* ══════════════════════════════════════════════════════════════════════
   Header
   ══════════════════════════════════════════════════════════════════════ */
.header-section {
    padding: 24px 20px 14px 20px;
}

.greeting {
    font-size: 23px;
    font-weight: 700;
    color: #ede8f8;
    letter-spacing: -0.3px;
}

.date-label {
    font-size: 12px;
    font-weight: 500;
    color: #7868a0;
    margin-top: 3px;
    letter-spacing: 0.2px;
}

/* ── Sidebar Toggle & Search Icon Buttons ── */
.sidebar-toggle,
button.image-button {
    min-width: 34px;
    min-height: 34px;
    padding: 0;
    border-radius: 10px;
    background-color: rgba(160, 130, 220, 0.07);
    border: 1px solid rgba(160, 130, 220, 0.1);
    color: #9888c8;
    transition: all 180ms ease;
}

.sidebar-toggle:hover,
button.image-button:hover {
    background-color: rgba(160, 130, 220, 0.14);
    border-color: rgba(180, 155, 235, 0.25);
    color: #c0aae8;
}

.sidebar-toggle:active,
button.image-button:active {
    background-color: rgba(160, 130, 220, 0.22);
}

/* ══════════════════════════════════════════════════════════════════════
   Sidebar
   ══════════════════════════════════════════════════════════════════════ */
.sidebar-container {
    background-color: rgba(8, 6, 18, 0.94);
    border-right: 1px solid rgba(160, 130, 220, 0.1);
}

.sidebar-header-title {
    font-size: 10px;
    font-weight: 800;
    color: #7868a0;
    text-transform: uppercase;
    letter-spacing: 1.2px;
}

.sidebar-list {
    background: transparent;
}

.sidebar-list row {
    padding: 2px 8px;
    border-radius: 10px;
    background: transparent;
}

.sidebar-row-box {
    padding: 9px 10px;
    border-radius: 10px;
    transition: background-color 150ms ease;
}

.sidebar-list row:selected .sidebar-row-box,
.sidebar-list row:selected {
    background-color: rgba(160, 130, 220, 0.15);
}

.sidebar-row-box:hover {
    background-color: rgba(160, 130, 220, 0.08);
}

.sidebar-add-entry {
    background-color: rgba(28, 22, 48, 0.7);
    border: 1px solid rgba(160, 130, 220, 0.18);
    border-radius: 8px;
    color: #c8bee8;
    padding: 6px 10px;
    transition: border-color 180ms ease;
}

.sidebar-add-entry:focus {
    border-color: rgba(180, 155, 235, 0.5);
}

/* ══════════════════════════════════════════════════════════════════════
   Task List
   ══════════════════════════════════════════════════════════════════════ */
.task-list {
    background: transparent;
}

.task-list row {
    padding: 2px 8px;
    margin: 1px 0;
    border-radius: 12px;
    background: transparent;
    transition: background-color 160ms ease;
}

.task-list row:hover {
    background-color: rgba(160, 130, 220, 0.06);
}

.task-row-box {
    padding: 10px 14px;
}

/* ── Task Title ── */
.task-title {
    font-size: 14px;
    font-weight: 500;
    color: #ede8f8;
    letter-spacing: -0.1px;
}

.task-notes-preview {
    font-size: 12px;
    color: #7868a0;
    margin-top: 2px;
}

/* ── Completed state ── */
.task-completed .task-title {
    text-decoration: line-through;
    color: #5c5478;
}

.task-completed .task-notes-preview {
    color: #483c60;
}

/* ── Checkbutton ── */
checkbutton check {
    min-width: 18px;
    min-height: 18px;
    border-radius: 50%;
    border: 2px solid rgba(160, 130, 220, 0.35);
    background-color: transparent;
    transition: all 200ms ease;
}

checkbutton:checked check {
    background-color: #b49ee0;
    border-color: #b49ee0;
}

/* ── Delete Button ── */
.task-delete {
    min-width: 28px;
    min-height: 28px;
    padding: 0;
    border-radius: 8px;
    color: #d08898;
    background: transparent;
    border: none;
    opacity: 0;
    transition: all 180ms ease;
}

.task-list row:hover .task-delete {
    opacity: 0.5;
}

.task-delete:hover {
    opacity: 1 !important;
    background-color: rgba(208, 136, 152, 0.12);
}

/* ══════════════════════════════════════════════════════════════════════
   Chips & Badges
   ══════════════════════════════════════════════════════════════════════ */
.meta-box {
    margin-top: 4px;
}

.due-chip {
    padding: 1px 7px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 600;
    color: #9888c8;
    background-color: rgba(152, 136, 200, 0.12);
}

.due-chip.overdue {
    background-color: rgba(208, 136, 152, 0.14);
    color: #d08898;
    border: 1px solid rgba(208, 136, 152, 0.22);
}

.priority-badge {
    padding: 1px 7px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.3px;
}

.priority-badge.priority-low {
    background-color: rgba(130, 200, 160, 0.1);
    color: #82c8a0;
}

.priority-badge.priority-medium {
    background-color: rgba(200, 168, 110, 0.12);
    color: #c8a86e;
}

.priority-badge.priority-high {
    background-color: rgba(208, 136, 152, 0.14);
    color: #d08898;
    border: 1px solid rgba(208, 136, 152, 0.22);
}

/* ══════════════════════════════════════════════════════════════════════
   Search Bar
   ══════════════════════════════════════════════════════════════════════ */
.search-container {
    margin: 6px 12px;
    padding: 10px;
    border-radius: 14px;
    background-color: rgba(18, 14, 32, 0.96);
    border: 1px solid rgba(160, 130, 220, 0.2);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.35);
}

.search-bar-entry {
    background-color: rgba(10, 8, 20, 0.6);
    border-radius: 8px;
    color: #ede8f8;
    border: 1px solid rgba(160, 130, 220, 0.15);
    padding: 6px 10px;
    transition: border-color 180ms ease;
}

.search-bar-entry:focus {
    border-color: rgba(180, 155, 235, 0.45);
}

.search-result-row {
    padding: 4px 0;
    border-radius: 8px;
}

/* ══════════════════════════════════════════════════════════════════════
   FAB (Floating Action Button)
   ══════════════════════════════════════════════════════════════════════ */
.fab {
    min-width: 50px;
    min-height: 50px;
    border-radius: 16px;
    background-color: #b49ee0;
    color: #0e0820;
    border: none;
    box-shadow:
        0 6px 20px rgba(180, 158, 224, 0.35),
        0 2px 6px rgba(0, 0, 0, 0.3);
    transition: all 240ms cubic-bezier(0.34, 1.56, 0.64, 1);
}

.fab:hover {
    background-color: #c8b4f0;
    box-shadow:
        0 8px 26px rgba(200, 180, 240, 0.45),
        0 2px 8px rgba(0, 0, 0, 0.35);
    transform: scale(1.06);
}

.fab:active {
    transform: scale(0.94);
    transition-duration: 100ms;
}

/* ══════════════════════════════════════════════════════════════════════
   Completed Tasks Section Toggle
   ══════════════════════════════════════════════════════════════════════ */
.completed-toggle {
    padding: 10px 22px;
    font-size: 11px;
    font-weight: 700;
    color: #5c5478;
    letter-spacing: 0.6px;
    text-transform: uppercase;
    background: transparent;
    border: none;
    transition: color 180ms ease;
}

.completed-toggle:hover {
    color: #9888c8;
}

/* ══════════════════════════════════════════════════════════════════════
   Empty State
   ══════════════════════════════════════════════════════════════════════ */
.empty-state {
    padding: 48px 24px;
}

.empty-icon {
    font-size: 38px;
    margin-bottom: 14px;
    opacity: 0.45;
}

.empty-title {
    font-size: 15px;
    font-weight: 700;
    color: #5c5478;
}

.empty-subtitle {
    font-size: 12px;
    color: #3e3458;
    margin-top: 5px;
}

/* ══════════════════════════════════════════════════════════════════════
   Task Editor Dialog
   ══════════════════════════════════════════════════════════════════════ */
.editor-title-entry {
    font-size: 15px;
    border-radius: 8px;
    padding: 8px 12px;
}

.editor-notes-scroll {
    border-radius: 8px;
    border: 1px solid rgba(160, 130, 220, 0.15);
    background-color: rgba(10, 8, 20, 0.5);
}

.editor-notes-view {
    background-color: transparent;
    color: #c8bee8;
    font-size: 13px;
    padding: 8px;
}

.editor-calendar {
    border-radius: 10px;
}

/* ══════════════════════════════════════════════════════════════════════
   Separator
   ══════════════════════════════════════════════════════════════════════ */
separator {
    background-color: rgba(160, 130, 220, 0.08);
    min-height: 1px;
    margin: 0 12px;
}
"#;

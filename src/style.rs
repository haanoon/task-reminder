/// Application stylesheet.
///
/// Color palette: Catppuccin Mocha — popular in the Hyprland community.
/// All values are GTK4-CSS–compatible (no web-only properties).
pub const CSS: &str = r#"
/* ══════════════════════════════════════════════════════════════════════
   Window — fully transparent so the wallpaper shows through
   ══════════════════════════════════════════════════════════════════════ */
window,
window.background {
    background-color: transparent;
}

/* ══════════════════════════════════════════════════════════════════════
   Main container — semi-transparent panel with rounded corners
   ══════════════════════════════════════════════════════════════════════ */
.main-container {
    background-color: rgba(30, 30, 46, 0.88);
    border-radius: 20px;
    border: 1px solid rgba(205, 214, 244, 0.06);
}

/* ══════════════════════════════════════════════════════════════════════
   Header (greeting + date)
   ══════════════════════════════════════════════════════════════════════ */
.header-section {
    padding: 28px 24px 12px 24px;
}

.greeting {
    font-size: 24px;
    font-weight: 800;
    color: #cdd6f4;
}

.date-label {
    font-size: 14px;
    font-weight: 500;
    color: #a6adc8;
    margin-top: 2px;
}

/* ══════════════════════════════════════════════════════════════════════
   List tabs — horizontal row of toggle-able list buttons
   ══════════════════════════════════════════════════════════════════════ */
.list-tabs {
    padding: 8px 16px 12px 16px;
}

.list-tab {
    padding: 6px 14px;
    border-radius: 10px;
    font-size: 13px;
    font-weight: 600;
    color: #a6adc8;
    background-color: rgba(49, 50, 68, 0.35);
    border: 1px solid transparent;
    min-height: 32px;
    transition: all 200ms ease-in-out;
}

.list-tab:hover {
    background-color: rgba(69, 71, 90, 0.55);
    color: #cdd6f4;
}

.list-tab.active {
    background-color: rgba(137, 180, 250, 0.15);
    color: #89b4fa;
    border-color: rgba(137, 180, 250, 0.25);
}

/* + New list button */
.add-list-btn {
    min-width: 32px;
    min-height: 32px;
    border-radius: 10px;
    padding: 0;
    color: #585b70;
    background: transparent;
    transition: all 200ms ease-in-out;
}

.add-list-btn:hover {
    color: #cdd6f4;
    background-color: rgba(69, 71, 90, 0.4);
}

/* ══════════════════════════════════════════════════════════════════════
   Separator
   ══════════════════════════════════════════════════════════════════════ */
.task-separator {
    min-height: 1px;
    background-color: rgba(69, 71, 90, 0.25);
    margin: 0 24px;
}

/* ══════════════════════════════════════════════════════════════════════
   Task list (ListBox)
   ══════════════════════════════════════════════════════════════════════ */
.task-list {
    background: transparent;
}

.task-list row {
    padding: 0;
    margin: 1px 8px;
    border-radius: 12px;
    background: transparent;
    transition: background-color 150ms ease-in-out;
}

.task-list row:hover {
    background-color: rgba(69, 71, 90, 0.30);
}

/* ── Row content ─────────────────────────────────────────────────── */
.task-row-box {
    padding: 10px 14px;
}

.task-title {
    font-size: 14px;
    font-weight: 500;
    color: #cdd6f4;
}

.task-completed .task-title {
    text-decoration: line-through;
    color: rgba(108, 112, 134, 0.60);
}

/* ── Delete button — hidden until row is hovered ────────────────── */
.task-delete {
    min-width: 28px;
    min-height: 28px;
    padding: 0;
    border-radius: 8px;
    color: #f38ba8;
    background: transparent;
    opacity: 0;
    transition: all 200ms ease-in-out;
}

.task-list row:hover .task-delete {
    opacity: 0.50;
}

.task-delete:hover {
    opacity: 1;
    background-color: rgba(243, 139, 168, 0.12);
}

/* ══════════════════════════════════════════════════════════════════════
   Completed section toggle
   ══════════════════════════════════════════════════════════════════════ */
.completed-toggle {
    padding: 10px 24px;
    font-size: 12px;
    font-weight: 700;
    color: #585b70;
    letter-spacing: 0.4px;
    background: transparent;
    border-radius: 0;
    transition: color 200ms ease-in-out;
}

.completed-toggle:hover {
    color: #a6adc8;
}

/* ══════════════════════════════════════════════════════════════════════
   Inline add-task entry
   ══════════════════════════════════════════════════════════════════════ */
.add-task-entry {
    margin: 8px 16px;
    padding: 10px 14px;
    border-radius: 12px;
    font-size: 14px;
    background-color: rgba(49, 50, 68, 0.55);
    color: #cdd6f4;
    border: 1px solid rgba(137, 180, 250, 0.20);
    transition: border-color 200ms ease-in-out;
}

.add-task-entry:focus {
    border-color: rgba(137, 180, 250, 0.60);
}

/* ══════════════════════════════════════════════════════════════════════
   New-list entry (appears below list tabs)
   ══════════════════════════════════════════════════════════════════════ */
.new-list-entry {
    min-height: 32px;
    margin: 0px 16px 8px 16px;
    padding: 4px 12px;
    border-radius: 10px;
    font-size: 13px;
    background-color: rgba(49, 50, 68, 0.65);
    color: #cdd6f4;
    border: 1px solid rgba(137, 180, 250, 0.30);
    transition: border-color 200ms ease-in-out;
}

.new-list-entry:focus {
    border-color: rgba(137, 180, 250, 0.60);
}

/* ══════════════════════════════════════════════════════════════════════
   FAB (Floating Action Button)
   ══════════════════════════════════════════════════════════════════════ */
.fab {
    min-width: 52px;
    min-height: 52px;
    border-radius: 16px;
    background-color: #89b4fa;
    color: #1e1e2e;
    box-shadow: 0 4px 16px rgba(137, 180, 250, 0.30);
    transition: all 200ms ease-in-out;
}

.fab:hover {
    background-color: #b4d0fb;
    box-shadow: 0 6px 20px rgba(137, 180, 250, 0.45);
}

.fab:active {
    background-color: #74c7ec;
}

/* ══════════════════════════════════════════════════════════════════════
   Empty state
   ══════════════════════════════════════════════════════════════════════ */
.empty-state {
    padding: 48px 24px;
}

.empty-icon {
    font-size: 40px;
    margin-bottom: 12px;
    opacity: 0.5;
}

.empty-title {
    font-size: 16px;
    font-weight: 600;
    color: #585b70;
}

.empty-subtitle {
    font-size: 13px;
    color: rgba(108, 112, 134, 0.60);
    margin-top: 4px;
}

/* ══════════════════════════════════════════════════════════════════════
   Scrollbar — thin and translucent
   ══════════════════════════════════════════════════════════════════════ */
/* ══════════════════════════════════════════════════════════════════════
   Task Metadata (Notes, Priority, Due Date)
   ══════════════════════════════════════════════════════════════════════ */
.task-notes-preview {
    font-size: 12px;
    color: #a6adc8;
    margin-top: 2px;
}

.task-completed .task-notes-preview {
    color: rgba(108, 112, 134, 0.40);
}

.meta-box {
    margin-top: 4px;
}

/* Due date chip */
.due-chip {
    padding: 2px 8px;
    border-radius: 6px;
    font-size: 11px;
    font-weight: 700;
    color: #cdd6f4;
    background-color: rgba(49, 50, 68, 0.5);
}

.due-chip.overdue {
    background-color: rgba(243, 139, 168, 0.15);
    color: #f38ba8;
    border: 1px solid rgba(243, 139, 168, 0.25);
}

/* Priority badge */
.priority-badge {
    padding: 2px 6px;
    border-radius: 6px;
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
}

.priority-badge.priority-low {
    background-color: rgba(137, 180, 250, 0.15);
    color: #89b4fa;
}

.priority-badge.priority-medium {
    background-color: rgba(250, 179, 135, 0.15);
    color: #fab387;
}

.priority-badge.priority-high {
    background-color: rgba(243, 139, 168, 0.2);
    color: #f38ba8;
    border: 1px solid rgba(243, 139, 168, 0.3);
}

/* ══════════════════════════════════════════════════════════════════════
   Task Editor Dialog Styling
   ══════════════════════════════════════════════════════════════════════ */
.dim-label {
    font-size: 12px;
    font-weight: 700;
    color: #585b70;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 2px;
}

.editor-title-entry {
    padding: 10px;
    border-radius: 8px;
    background-color: rgba(49, 50, 68, 0.4);
    color: #cdd6f4;
}

.editor-notes-scroll {
    border-radius: 8px;
    background-color: rgba(49, 50, 68, 0.4);
    border: 1px solid rgba(205, 214, 244, 0.05);
}

.editor-notes-view {
    padding: 8px;
    background: transparent;
    color: #cdd6f4;
}

.editor-calendar {
    border-radius: 10px;
    background-color: rgba(49, 50, 68, 0.3);
    padding: 6px;
}

.destructive-action {
    color: #f38ba8;
}

.destructive-action:hover {
    background-color: rgba(243, 139, 168, 0.1);
}

/* ══════════════════════════════════════════════════════════════════════
   Sidebar Layout
   ══════════════════════════════════════════════════════════════════════ */
.sidebar-container {
    background-color: rgba(30, 30, 46, 0.95);
    border-right: 1px solid rgba(205, 214, 244, 0.05);
}

.sidebar-header-title {
    font-size: 16px;
    font-weight: 800;
    color: #cdd6f4;
}

.sidebar-list {
    background: transparent;
}

.sidebar-row-box {
    padding: 8px 12px;
}

.sidebar-delete-btn {
    min-width: 24px;
    min-height: 24px;
    padding: 0;
    color: #f38ba8;
    background: transparent;
}

.sidebar-delete-btn:hover {
    background-color: rgba(243, 139, 168, 0.1);
}

.sidebar-add-entry {
    background-color: rgba(49, 50, 68, 0.4);
    border-radius: 6px;
    color: #cdd6f4;
}

/* ══════════════════════════════════════════════════════════════════════
   Search Overlay Layout
   ══════════════════════════════════════════════════════════════════════ */
.search-container {
    margin: 8px 16px;
    padding: 10px;
    border-radius: 12px;
    background-color: rgba(30, 30, 46, 0.9);
    border: 1px solid rgba(137, 180, 250, 0.2);
}

.search-bar-entry {
    background-color: rgba(49, 50, 68, 0.5);
    border-radius: 8px;
    color: #cdd6f4;
}

.search-results-list {
    background: transparent;
}

.search-result-row {
    padding: 8px;
    border-bottom: 1px solid rgba(205, 214, 244, 0.02);
}

.search-result-row:hover {
    background-color: rgba(69, 71, 90, 0.2);
}

.search-result-title {
    font-size: 13px;
    color: #cdd6f4;
}

scrollbar slider {
    background-color: rgba(108, 112, 134, 0.20);
    border-radius: 100px;
    min-width: 4px;
    min-height: 20px;
}

scrollbar slider:hover {
    background-color: rgba(108, 112, 134, 0.40);
}
"#;

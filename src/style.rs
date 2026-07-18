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

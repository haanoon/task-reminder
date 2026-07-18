/// Premium Space Obsidian & Electric Cyan Stylesheet.
/// Uses vibrant glassmorphism, soft glow, and modern typography variables.
pub const CSS: &str = r#"
/* ══════════════════════════════════════════════════════════════════════
   Window
   ══════════════════════════════════════════════════════════════════════ */
window,
window.background {
    background-color: transparent;
}

/* ══════════════════════════════════════════════════════════════════════
   Main Glass Container
   ══════════════════════════════════════════════════════════════════════ */
.main-container {
    background-color: rgba(17, 17, 27, 0.78);
    border-radius: 24px;
    border: 1px solid rgba(137, 220, 235, 0.15);
    box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.4);
}

/* ══════════════════════════════════════════════════════════════════════
   Header (Greeting + Date)
   ══════════════════════════════════════════════════════════════════════ */
.header-section {
    padding: 30px 24px 16px 24px;
}

.greeting {
    font-size: 26px;
    font-weight: 800;
    color: #89dceb; /* Electric Cyan */
    text-shadow: 0 0 10px rgba(137, 220, 235, 0.3);
}

.date-label {
    font-size: 13px;
    font-weight: 600;
    color: #a6adc8;
    margin-top: 4px;
    letter-spacing: 0.3px;
}

/* ══════════════════════════════════════════════════════════════════════
   Sidebar navigation split view
   ══════════════════════════════════════════════════════════════════════ */
.sidebar-container {
    background-color: rgba(10, 10, 15, 0.92);
    border-right: 1px solid rgba(137, 220, 235, 0.1);
}

.sidebar-header-title {
    font-size: 15px;
    font-weight: 800;
    color: #89dceb;
    text-transform: uppercase;
    letter-spacing: 0.8px;
}

.sidebar-row-box {
    padding: 10px 14px;
    border-radius: 8px;
    transition: all 150ms ease;
}

.sidebar-row-box:hover {
    background-color: rgba(137, 220, 235, 0.08);
}

.sidebar-add-entry {
    background-color: rgba(30, 30, 46, 0.6);
    border: 1px solid rgba(137, 220, 235, 0.2);
    border-radius: 8px;
    color: #cdd6f4;
    padding: 6px 10px;
}

.sidebar-add-entry:focus {
    border-color: #89dceb;
}

/* ══════════════════════════════════════════════════════════════════════
   Task List
   ══════════════════════════════════════════════════════════════════════ */
.task-list row {
    padding: 4px;
    margin: 2px 10px;
    border-radius: 14px;
    background: transparent;
    transition: all 200ms ease;
}

.task-list row:hover {
    background-color: rgba(137, 220, 235, 0.05);
}

.task-row-box {
    padding: 12px 16px;
}

.task-title {
    font-size: 15px;
    font-weight: 500;
    color: #cdd6f4;
}

.task-completed .task-title {
    text-decoration: line-through;
    color: rgba(180, 190, 254, 0.45); /* Soft Lavender Grey completed state */
}

/* ── Delete Button ── */
.task-delete {
    min-width: 30px;
    min-height: 30px;
    padding: 0;
    border-radius: 8px;
    color: #f38ba8; /* Pastel Hot Pink */
    background: transparent;
    opacity: 0;
    transition: all 200ms ease;
}

.task-list row:hover .task-delete {
    opacity: 0.6;
}

.task-delete:hover {
    opacity: 1;
    background-color: rgba(243, 139, 168, 0.15);
}

/* ══════════════════════════════════════════════════════════════════════
   Chips & Badges
   ══════════════════════════════════════════════════════════════════════ */
.due-chip {
    padding: 2px 8px;
    border-radius: 8px;
    font-size: 11px;
    font-weight: 700;
    color: #89dceb;
    background-color: rgba(137, 220, 235, 0.1);
}

.due-chip.overdue {
    background-color: rgba(243, 139, 168, 0.12);
    color: #f38ba8;
    border: 1px solid rgba(243, 139, 168, 0.2);
}

.priority-badge {
    padding: 2px 8px;
    border-radius: 8px;
    font-size: 10px;
    font-weight: 800;
    text-transform: uppercase;
}

.priority-badge.priority-low {
    background-color: rgba(180, 190, 254, 0.15);
    color: #b4befe;
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
   Search Overlay
   ══════════════════════════════════════════════════════════════════════ */
.search-container {
    margin: 8px 16px;
    padding: 12px;
    border-radius: 16px;
    background-color: rgba(24, 24, 37, 0.95);
    border: 1px solid rgba(137, 220, 235, 0.25);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.3);
}

.search-bar-entry {
    background-color: rgba(17, 17, 27, 0.6);
    border-radius: 8px;
    color: #cdd6f4;
}

/* ══════════════════════════════════════════════════════════════════════
   FAB (Floating Action Button)
   ══════════════════════════════════════════════════════════════════════ */
.fab {
    min-width: 54px;
    min-height: 54px;
    border-radius: 18px;
    background-color: #89dceb;
    color: #11111b;
    box-shadow: 0 8px 24px rgba(137, 220, 235, 0.35);
    transition: all 250ms cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.fab:hover {
    background-color: #a6e3a1; /* Pastel Green on hover */
    box-shadow: 0 10px 28px rgba(166, 227, 161, 0.45);
    transform: scale(1.05);
}

.fab:active {
    transform: scale(0.95);
}

/* ══════════════════════════════════════════════════════════════════════
   Completed Toggle Section
   ══════════════════════════════════════════════════════════════════════ */
.completed-toggle {
    padding: 12px 24px;
    font-size: 12px;
    font-weight: 800;
    color: #b4befe;
    letter-spacing: 0.5px;
    background: transparent;
    transition: color 200ms ease;
}

.completed-toggle:hover {
    color: #89dceb;
}

/* ══════════════════════════════════════════════════════════════════════
   Empty State
   ══════════════════════════════════════════════════════════════════════ */
.empty-state {
    padding: 56px 24px;
}

.empty-icon {
    font-size: 44px;
    margin-bottom: 16px;
    opacity: 0.7;
}

.empty-title {
    font-size: 17px;
    font-weight: 700;
    color: #89dceb;
}

.empty-subtitle {
    font-size: 13px;
    color: #a6adc8;
    margin-top: 6px;
}
"#;

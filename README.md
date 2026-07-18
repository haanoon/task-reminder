# Wallpaper Tasks

A beautiful, dark-themed task manager built specifically for **Hyprland**. 

Instead of opening a heavy standalone app, **Wallpaper Tasks** sits seamlessly over your desktop wallpaper. You can instantly summon or hide it with a single keypress, keeping your workspace clean while keeping your tasks just a keystroke away. Built with GTK4 and Libadwaita for a sleek, premium Wayland experience.

## How to use this
Once built and installed, you can launch or toggle the application using the command line:

- **Start / Bring to front**:
  ```bash
  wallpaper-tasks
  ```
- **Toggle visibility (Show/Hide)**:
  ```bash
  wallpaper-tasks --toggle
  ```

### Setting up a Keybind (Hyprland Example)
For the best experience, bind the toggle command to a shortcut in your window manager. For example, in Hyprland, you can add this to your configuration:
```conf
bind = SUPER, T, exec, wallpaper-tasks --toggle
```
This allows you to instantly summon or hide your tasks with a single keypress!

---
This is a vibe coded project.
Author: hanoon

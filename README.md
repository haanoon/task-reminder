# Wallpaper Tasks

A task manager that lives on your desktop wallpaper.

## What is this project?
Wallpaper Tasks is a sleek, lightweight task manager designed to integrate seamlessly into your desktop environment (especially Wayland compositors like Hyprland). It uses GTK4 and Libadwaita to provide a beautiful, dark-themed UI that sits right on your wallpaper, allowing you to manage your tasks without opening heavy standalone applications.

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

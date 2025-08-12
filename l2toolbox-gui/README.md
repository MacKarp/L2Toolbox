# l2toolbox-gui

Graphical user interface for the L2Toolbox project, created with the Iced toolkit.

This crate depends on l2toolbox-core and is part of the L2Toolbox workspace.

## 🛠 Configuration

The application uses a user-specific configuration file to store preferences and state.

### 📁 Config File Location
On Windows, the config file is stored at:

```
%APPDATA%\L2Toolbox\config\config.toml
```
---

### 🧾 Config Structure

```toml
last_profile = ""
language = "En"
```

---

### 🧰 Default Values

| Field         | Type   | Default | Description                          |
|---------------|--------|---------|--------------------------------------|
| `last_profile`| String | `""`    | Stores the name of the last used profile |
| `language`    | String | `"En"`  | Language preference (`"En"` or `"Pl"`) |

---

### 🔄 Behavior

- If the config file is **missing**, a default one is created automatically.
- If the config file is **corrupted**, the application will return an error during startup.
- All fields have default values to ensure compatibility with older or incomplete config files.
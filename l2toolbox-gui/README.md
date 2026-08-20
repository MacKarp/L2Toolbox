# l2toolbox-gui

Graphical user interface for the L2Toolbox project, created with the [Iced](https://iced.rs/) toolkit.

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
last_profile_id = ""
language = "en-GB"
```

---

### 🧰 Default Values

| Field         | Type   | Default | Description                          |
|---------------|--------|---------|--------------------------------------|
| `last_profile_id`| String | `""`    | Stores the UUID v4 of the last used profile |
| `language`    | String | `"en-GB"`  | Language preference (any valid BCP47 code corresponding to a `.ftl` file) |

---


### 🔄 Behavior

- If the config file is **missing**, a default one is created automatically.
- If the config file is **corrupted**, it is renamed to `config.toml_<timestamp>.bak` and a new default config is created automatically.
- All fields have default values to ensure compatibility with older or incomplete config files.
---

## 🛠 Profile Configuration
The application uses a user-specific profile configuration file to store information about different profiles.

### 📁 Profile Config File Location
On Windows, the profile config file is stored at:

```
%APPDATA%\L2Toolbox\profiles\<uuid>\profile_config.toml
```
---

### 🧾 Profile Config Structure

```toml
profile_name = ""
lineage2_path = ""
system_path = ""
custom_subdir = false
```
---

### 🧰 Default Values

| Field         | Type   | Default | Description                          |
|---------------|--------|---------|--------------------------------------|
| `profile_name`| String | `""`    | Display name of the profile |
| `lineage2_path`    | String | `""`  | Path to the Lineage 2 main directory |
|`system_path`|String (optional)|`""`|Path to the custom System directory (only when `custom_subdir` is true)|
|`custom_subdir`|Bool|`false`|Whether to use a custom System subdirectory|

---

## 🌐 Translation

Translations are managed with the [Fluent system](https://projectfluent.org/) and stored in the `Languages` directory.

- **Base file**: `en-GB.ftl`  
  This file acts as the template for all other translations. It is also used as the **fallback** when a key is missing in another language.
- **File naming**: All translation files must follow the [BCP47 standard](https://unicode.org/reports/tr35/tr35.html#BCP47), e.g. `pl-PL.ftl` for Polish.
- **Reserved files**:  
  - `xx-INVALID.ftl` and `xx-TEST.ftl` are used exclusively for automated tests. Do not modify or use them outside of testing.
- **Required key**: Each translation file must include `language-name` so that the application can display it in the language selector.

### ➕ Adding a New Translation

1. Copy `Languages/en-GB.ftl` to `Languages/<lang>.ftl`  
   (replace `<lang>` with a valid BCP47 code, e.g. `fr-FR`).
2. Translate all string values inside the new file.
3. Use a special key `language-name` to specify the name of the language in its own form, for example:

   ```fluent
   language-name = Français
   ```

4. Run the application. The new language will automatically be discovered and available for selection.

### 📝 Recommended Language Names (Examples)

| File      | language-name |
|-----------|---------------|
| en-GB.ftl | English (Great Britain) |
| pl-PL.ftl | Polski |
| fr-FR.ftl | Français |
| de-DE.ftl | Deutsch |
| es-ES.ftl | Español |

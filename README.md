# Egui Code Editor

Text Editor Widget for [egui](https://github.com/emilk/egui) with numbered lines and simple syntax highlighting based on keywords sets.

## Usage

```rust
use egui_code_editor::{CodeEditor, ColorTheme, Syntax};
CodeEditor::default()
  .id_source("code editor")
  .with_rows(12)
  .with_fontsize(14.0)
  .with_theme(ColorTheme::GRUVBOX)
  .with_syntax(Syntax::rust())
  .with_numlines(true)
  .show(ui, &mut self.code);
  });
}
```


## Themes

Based on themes in [Helix Editor](https://github.com/helix-editor/helix).

Font used in examples is [Comic Code](https://tosche.net/fonts/comic-code) by Toshi Omagari.

### Ayu
![Ayu](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/ayu.png)

### Ayu Dark
![Ayu Dark](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/ayu_dark.png)

### Ayu Mirage
![Ayu Mirage](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/ayu_mirage.png)

### Github Dark
![Github Dark](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/github_dark.png)

### Github Light
![Github Light](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/github_light.png)

### Gruvbox
![Gruvbox](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/gruvbox.png)

### Gruvbox Light
![Gruvbox Light](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/gruvbox_light.png)

### Sonokai
![Sonokai](url:https://github.com/p4ymak/egui_code_editor/blob/main/screenshots/sonokai.png)

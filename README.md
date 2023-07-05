# Egui Code Editor

## Themes

Based on themes in [Helix Editor](https://github.com/helix-editor/helix).

Font used in examples is [Comic Code](https://tosche.net/fonts/comic-code) by Toshi Omagari.

### Ayu
![Ayu](/screenshots/ayu.png)

### Ayu Dark
![Ayu Dark](/screenshots/ayu_dark.png)

### Ayu Mirage
![Ayu Mirage](/screenshots/ayu_mirage.png)

### Github Dark
![Github Dark](/screenshots/github_dark.png)

### Github Light
![Github Light](/screenshots/github_light.png)

### Gruvbox
![Gruvbox](/screenshots/gruvbox.png)

### Gruvbox Light
![Gruvbox Light](/screenshots/gruvbox_light.png)

### Sonokai
![Sonokai](/screenshots/sonokai.png)


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

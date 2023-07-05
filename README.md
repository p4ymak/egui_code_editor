# Egui Code Editor


## Usage:

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

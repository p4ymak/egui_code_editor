use crate::highlighting::Links;
use egui::{Pos2, Rect, text_edit::TextEditOutput};

pub const SPACE_HOLDER: &str = "␣";

pub fn handle_links(text_edit: &TextEditOutput, links: &Links) {
    if !text_edit.response.contains_pointer() {
        return;
    }
    let galley = &text_edit.galley;
    let top_left = text_edit.galley_pos.to_vec2();
    let ctx = &text_edit.response.ctx;

    let chars = galley.chars().collect::<Vec<char>>();
    links.iter().for_each(|link_range| {
        let link_start = link_range.start;
        let link_end = link_range.end;
        if let Some(url) = chars
            .get(link_start..link_end)
            .map(|c| c.iter().collect::<String>())
        {
            let cursors = (link_start..=link_end)
                .map(|index| {
                    galley.pos_from_cursor(egui::text::CCursor {
                        index: index.into(),
                        prefer_next_row: false,
                    })
                })
                .collect::<Vec<Rect>>();
            let rects = join_cursor_rects(&cursors, top_left);
            for rect in rects {
                if ctx.pointer_hover_pos().is_some_and(|p| rect.contains(p)) {
                    ctx.set_cursor_icon(egui::CursorIcon::PointingHand);

                    if ctx.input(|r| r.pointer.primary_pressed()) {
                        if url.to_lowercase().starts_with("file://") {
                            let path = &url[7..].replace(SPACE_HOLDER, " ");
                            opener::open(path)
                                .inspect_err(|e| {
                                    if cfg!(debug_assertions) {
                                        println!("{e:?}");
                                    }
                                })
                                .ok();
                        } else {
                            let url = if url.to_lowercase().starts_with("www") {
                                format!("https://{url}")
                            } else {
                                url.to_string()
                            };

                            ctx.open_url(egui::OpenUrl {
                                url: url.clone(),
                                new_tab: true,
                            });
                        }
                    }
                }
            }
        }
    });
}

fn join_cursor_rects(cursors: &[Rect], top_left: egui::Vec2) -> Vec<Rect> {
    let mut rects = Vec::<Rect>::new();
    if cursors.is_empty() {
        return rects;
    }
    let mut last = cursors.first().copied().expect("first exists");
    let mut start_x = last.min.x;
    let mut cursors = cursors.iter().peekable();

    while let Some(current) = cursors.next() {
        if cursors.peek().is_none_or(|n| n.min.y != last.min.y) {
            if current.min.y == last.min.y {
                last = *current;
            }
            rects.push(
                Rect::from_min_max(
                    Pos2::new(start_x, last.min.y),
                    Pos2::new(last.max.x, last.max.y),
                )
                .translate(top_left),
            );
            start_x = current.min.x;
        }
        last = *current;
    }

    rects
}

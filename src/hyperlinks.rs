use crate::highlighting::Links;
use egui::{Pos2, Rect};

pub fn handle_links(
    links: &Links,
    galley: &std::sync::Arc<egui::Galley>,
    ui: &egui::Ui,
    top_left: egui::Vec2,
) {
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
                        index,
                        prefer_next_row: false,
                    })
                })
                .collect::<Vec<Rect>>();
            let rects = join_cursor_rects(&cursors, top_left);
            for rect in rects {
                if ui.pointer_hover_pos().is_some_and(|p| rect.contains(p)) {
                    ui.set_cursor_icon(egui::CursorIcon::PointingHand);

                    if ui.input_mut(|r| r.pointer.primary_pressed()) {
                        ui.open_url(egui::OpenUrl {
                            url: url.clone(),
                            new_tab: true,
                        });
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

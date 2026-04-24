pub fn render_progress(scanned: u16, total: u16, open_count: u16) -> String {
    let bar_width: usize = 24;

    let ratio: f32 = if total == 0 {
        0.0
    } else {
        scanned as f32 / total as f32
    };

    let full_blocks: usize = (ratio * bar_width as f32).round() as usize;

    let mut bar = String::new();

    for _ in 0..full_blocks {
        bar.push('█');
    }

    while bar.chars().count() < bar_width {
        bar.push('·');
    }

    format!("⟦{}⟧ {}/{}  open: {}", bar, scanned, total, open_count)
}

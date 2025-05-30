pub fn generate_display_utilities() -> String {
    let display_types = vec![
        ("block", "block"),
        ("inline-block", "inline-block"),
        ("flex", "flex"),
        ("grid", "grid"),
        ("inline-flex", "inline-flex"),
        ("inline-grid", "inline-grid"),
    ];

    let mut css = String::new();

    for (class, value) in display_types {
        css.push_str(&format!(".{} {{ display: {}; }}\n", class, value));
    }

    css
}

pub fn generate_flex_direction_utilities() -> String {
    let directions = vec![
        ("row", "row"),
        ("col", "column"),
        ("row-reverse", "row-reverse"),
        ("col-reverse", "column-reverse"),
    ];

    let mut css = String::new();

    for (class, value) in directions {
        css.push_str(&format!(".flex-{} {{ flex-direction: {}; }}\n", class, value));
    }

    css
}

pub fn generate_justify_content_utilities() -> String {
    let alignments = vec![
        ("center", "center"),
        ("start", "flex-start"),
        ("end", "flex-end"),
        ("between", "space-between"),
        ("around", "space-around"),
        ("evenly", "space-evenly"),
    ];

    let mut css = String::new();

    for (class, value) in alignments {
        css.push_str(&format!(".justify-{} {{ justify-content: {}; }}\n", class, value));
    }

    css
}

pub fn generate_align_items_utilities() -> String {
    let alignments = vec![
        ("start", "flex-start"),
        ("center", "center"),
        ("end", "flex-end"),
        ("baseline", "baseline"),
        ("stretch", "stretch"),
    ];

    let mut css = String::new();

    for (class, value) in alignments {
        css.push_str(&format!(".items-{} {{ align-items: {}; }}\n", class, value));
    }

    css
}


pub fn generate_gap_utilities() -> String {
    let gaps = vec![0, 1, 2, 3, 4, 5, 6, 8, 10];

    let mut css = String::new();

    for gap in gaps {
        css.push_str(&format!(".gap-{} {{ gap: {}px; }}\n", gap, gap * 4)); // Adjust the multiplier as necessary
        css.push_str(&format!(".gap-x-{} {{ column-gap: {}px; }}\n", gap, gap * 4));
        css.push_str(&format!(".gap-y-{} {{ row-gap: {}px; }}\n", gap, gap * 4));
    }

    css
}

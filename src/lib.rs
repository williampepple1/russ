// src/lib.rs
mod utils;
use utils::bg::{generate_bg_color_utilities, generate_background_gradient_utilities};
use utils::text::generate_text_color_utilities;
use utils::font::{generate_font_size_utilities, generate_font_style_utilities, generate_font_weight_utilities, generate_text_alignment_utilities,  generate_line_height_utilities};
use utils::layout::{generate_padding_utilities, generate_margin_utilities, generate_height_utilities, generate_max_height_utilities, generate_width_utilities, generate_max_width_utilities, generate_min_height_utilities, generate_min_width_utilities};
use utils::border::{generate_border_radius_utilities, generate_border_width_utilities, generate_border_color_utilities, generate_border_style_utilities, generate_border_corner_radius_utilities, generate_shadow_utilities};
use utils::effect::{generate_hover_focus_utilities, generate_hover_focus_for_utilities, generate_dark_mode_utilities, generate_active_state_utilities};
use utils::responsive::wrap_with_responsive_variants;
use utils::direction::{generate_display_utilities, generate_flex_direction_utilities, generate_justify_content_utilities, generate_align_items_utilities};

// pub fn minify_css(css: &str) -> String {
//     css
//         .lines()
//         .map(|line| line.trim())     // Remove leading/trailing whitespace
//         .filter(|line| !line.is_empty()) // Skip empty lines
//         .collect::<Vec<_>>()
//         .join("")                     // Join without newlines
// }


pub fn generate_css() -> String {
    let base_css =    
    
     format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        generate_bg_color_utilities(),
        generate_padding_utilities(),
        generate_font_size_utilities(),
        generate_font_weight_utilities(),
        generate_font_style_utilities(),
        generate_margin_utilities(),
        generate_text_color_utilities(),
        generate_height_utilities(),
        generate_max_height_utilities(),
        generate_width_utilities(),
        generate_max_width_utilities(),
        generate_min_height_utilities(),
        generate_min_width_utilities(),
        generate_border_radius_utilities(),
        generate_border_width_utilities(),
        generate_border_color_utilities(),
        generate_border_style_utilities(),
        generate_border_corner_radius_utilities(),
        generate_shadow_utilities(),
        generate_background_gradient_utilities(),
        generate_hover_focus_utilities(),
        generate_text_alignment_utilities(),
        generate_line_height_utilities(),
        generate_display_utilities(),
        generate_flex_direction_utilities(),
        generate_justify_content_utilities(),
        generate_align_items_utilities()

    );
    
    let responsive_css = wrap_with_responsive_variants(&base_css);
    let hover_focus_css = generate_hover_focus_for_utilities();
    let dark_mode_css = generate_dark_mode_utilities();
    let active_state_css = generate_active_state_utilities();

//  let full_css =  
  format!(
        "{}{}{}{}{}",
        base_css,
        hover_focus_css,
        dark_mode_css,
        active_state_css,
        responsive_css
    )

    // minify_css(&full_css) 

    
}

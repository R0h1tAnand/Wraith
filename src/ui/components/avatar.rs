use dioxus::prelude::*;
use sha2::{Digest, Sha256};
use crate::ui::theme::DARK;

/// Deterministic identicon avatar generated from a public key.
///
/// Creates a 5×5 symmetric grid pattern with colors derived from the key hash.
#[component]
pub fn Avatar(pubkey: String, size: u32) -> Element {
    let hash = Sha256::digest(pubkey.as_bytes());

    // Derive color from first 3 bytes
    let hue = (hash[0] as u16 * 360 / 255) as u16;
    let sat = 50 + (hash[1] % 30) as u16;
    let light = 55 + (hash[2] % 15) as u16;
    let color = format!("hsl({}, {}%, {}%)", hue, sat, light);
    let bg_color = format!("hsl({}, {}%, {}%)", hue, sat / 3, light / 4);

    // Generate 5×5 grid (mirrored left-right for symmetry)
    let mut grid = [[false; 5]; 5];
    for row in 0..5 {
        for col in 0..3 {
            let byte_idx = 3 + row * 3 + col;
            let active = hash[byte_idx] > 127;
            grid[row][col] = active;
            grid[row][4 - col] = active; // Mirror
        }
    }

    let cell_size = size as f32 / 7.0; // 5 cells + 1 cell padding on each side
    let offset = cell_size;

    rsx! {
        svg {
            width: "{size}",
            height: "{size}",
            view_box: "0 0 {size} {size}",
            xmlns: "http://www.w3.org/2000/svg",

            // Background circle
            circle {
                cx: "{size / 2}",
                cy: "{size / 2}",
                r: "{size / 2}",
                fill: "{bg_color}",
            }

            // Grid cells
            for row in 0..5 {
                for col in 0..5 {
                    if grid[row][col] {
                        rect {
                            x: "{offset + col as f32 * cell_size}",
                            y: "{offset + row as f32 * cell_size}",
                            width: "{cell_size}",
                            height: "{cell_size}",
                            rx: "{cell_size * 0.2}",
                            fill: "{color}",
                        }
                    }
                }
            }
        }
    }
}

use dioxus::prelude::*;
use sha2::{Digest, Sha256};
use crate::ui::theme::DARK;

/// Simple QR code display component.
///
/// Generates a basic QR-like pattern from the data hash.
/// For a production app, this should use a proper QR encoding library.
#[component]
pub fn QrCode(data: String, size: u32) -> Element {
    let hash = Sha256::digest(data.as_bytes());

    // Generate a deterministic grid pattern from the hash
    // This creates a visual representation, not a scannable QR code
    let grid_size = 21; // Standard QR Version 1 size
    let cell_size = size as f32 / grid_size as f32;

    // Build the grid from hash bytes (repeated as needed)
    let mut grid = vec![vec![false; grid_size]; grid_size];

    for row in 0..grid_size {
        for col in 0..grid_size {
            let idx = (row * grid_size + col) % 32;
            let bit = (row * grid_size + col) % 8;
            grid[row][col] = (hash[idx] >> bit) & 1 == 1;
        }
    }

    // Add finder patterns (top-left, top-right, bottom-left)
    add_finder_pattern(&mut grid, 0, 0);
    add_finder_pattern(&mut grid, 0, grid_size - 7);
    add_finder_pattern(&mut grid, grid_size - 7, 0);

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                justify-content: center;
            ",

            svg {
                width: "{size}",
                height: "{size}",
                view_box: "0 0 {size} {size}",
                xmlns: "http://www.w3.org/2000/svg",

                // White background
                rect {
                    x: "0",
                    y: "0",
                    width: "{size}",
                    height: "{size}",
                    rx: "8",
                    fill: "white",
                }

                // Grid cells
                for row in 0..grid_size {
                    for col in 0..grid_size {
                        if grid[row][col] {
                            rect {
                                x: "{col as f32 * cell_size}",
                                y: "{row as f32 * cell_size}",
                                width: "{cell_size + 0.5}",
                                height: "{cell_size + 0.5}",
                                fill: "#1a1a2e",
                            }
                        }
                    }
                }

                // Wraith logo overlay in center
                circle {
                    cx: "{size / 2}",
                    cy: "{size / 2}",
                    r: "{size / 8}",
                    fill: "white",
                }
                circle {
                    cx: "{size / 2}",
                    cy: "{size / 2}",
                    r: "{size / 10}",
                    fill: DARK.accent_primary,
                }
                // Ghost emoji as text in center
                text {
                    x: "{size / 2}",
                    y: "{size / 2 + 3}",
                    text_anchor: "middle",
                    dominant_baseline: "middle",
                    font_size: "{size / 8}",
                    "👻"
                }
            }
        }
    }
}

/// Add a 7×7 finder pattern at the given position.
fn add_finder_pattern(grid: &mut Vec<Vec<bool>>, row: usize, col: usize) {
    for r in 0..7 {
        for c in 0..7 {
            let is_border = r == 0 || r == 6 || c == 0 || c == 6;
            let is_inner = r >= 2 && r <= 4 && c >= 2 && c <= 4;
            if row + r < grid.len() && col + c < grid[0].len() {
                grid[row + r][col + c] = is_border || is_inner;
            }
        }
    }
}

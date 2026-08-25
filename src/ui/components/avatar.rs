use dioxus::prelude::*;
use crate::ui::theme::DARK;

/// Deterministic avatar identicon with gradient glow ring.
///
/// Generates a unique visual fingerprint from the public key hash,
/// wrapped in a luminous gradient ring.
#[component]
pub fn Avatar(
    /// Hex-encoded public key (or any unique identifier).
    pubkey: String,
    /// Diameter in pixels (default 44).
    #[props(default = 44)]
    size: u32,
) -> Element {
    let hash = simple_hash(&pubkey);
    let hue = hash % 360;
    let cells = generate_identicon(hash);
    let cell_size = size as f64 / 5.0;
    let ring_size = size + 6;

    rsx! {
        div {
            style: "
                position: relative;
                width: {ring_size}px;
                height: {ring_size}px;
                display: flex;
                align-items: center;
                justify-content: center;
                flex-shrink: 0;
            ",

            // Gradient ring
            div {
                style: "
                    position: absolute;
                    inset: 0;
                    border-radius: 50%;
                    background: {gradient};
                    opacity: 0.6;
                ",
            }

            // Outer glow
            div {
                style: "
                    position: absolute;
                    inset: -4px;
                    border-radius: 50%;
                    background: transparent;
                    box-shadow: 0 0 16px hsla({hue}, 70%, 60%, 0.2);
                ",
            }

            // Inner identicon circle
            div {
                style: "
                    width: {size}px;
                    height: {size}px;
                    border-radius: 50%;
                    background: {bg};
                    position: relative;
                    overflow: hidden;
                    z-index: 1;
                ",
                bg = DARK.bg_secondary,

                // Identicon grid
                svg {
                    width: "{size}",
                    height: "{size}",
                    view_box: "0 0 {size} {size}",

                    for (i, &filled) in cells.iter().enumerate() {
                        if filled {
                            {
                                let row = i / 5;
                                let col = i % 5;
                                let x = col as f64 * cell_size;
                                let y = row as f64 * cell_size;
                                let lightness = 55 + ((i * 7) % 20);
                                rsx! {
                                    rect {
                                        x: "{x}",
                                        y: "{y}",
                                        width: "{cell_size}",
                                        height: "{cell_size}",
                                        fill: "hsla({hue}, 65%, {lightness}%, 0.85)",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn simple_hash(input: &str) -> u32 {
    input.bytes().fold(5381u32, |acc, b| {
        acc.wrapping_mul(33).wrapping_add(b as u32)
    })
}

fn generate_identicon(hash: u32) -> [bool; 25] {
    let mut cells = [false; 25];
    for row in 0..5 {
        for col in 0..3 {
            let bit = (hash >> ((row * 3 + col) % 32)) & 1 == 1;
            cells[row * 5 + col] = bit;
            cells[row * 5 + (4 - col)] = bit; // mirror
        }
    }
    cells
}

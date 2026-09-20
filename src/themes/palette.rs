//! Color palettes and theme registry for TermLoom.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    pub name: &'static str,
    pub fg: (u8, u8, u8),
    pub bg: (u8, u8, u8),
    pub cursor: (u8, u8, u8),
    pub ansi: [(u8, u8, u8); 16],
}

impl Palette {
    /// Convert an RGB tuple to a hex string like `#1e1e2e`.
    pub fn hex(c: (u8, u8, u8)) -> String {
        format!("#{:02x}{:02x}{:02x}", c.0, c.1, c.2)
    }

    /// Resolve an ANSI 256-color index into an RGB tuple.
    pub fn resolve_256(&self, idx: u8) -> (u8, u8, u8) {
        match idx {
            0..=15 => self.ansi[idx as usize],
            16..=231 => {
                // 6x6x6 color cube
                let idx = idx - 16;
                let r = (idx / 36) % 6;
                let g = (idx / 6) % 6;
                let b = idx % 6;
                let conv = |v: u8| if v == 0 { 0 } else { 55 + v * 40 };
                (conv(r), conv(g), conv(b))
            }
            232..=255 => {
                // Grayscale ramp from 232 (dark) to 255 (light)
                let v = 8 + (idx - 232) * 10;
                (v, v, v)
            }
        }
    }
}

pub static CATPPUCCIN_MOCHA: Palette = Palette {
    name: "catppuccin-mocha",
    fg: (205, 214, 244),     // #cdd6f4
    bg: (30, 30, 46),        // #1e1e2e
    cursor: (245, 224, 220), // #f5e0dc
    ansi: [
        (69, 71, 90),    // 0: Black (Surface1)
        (243, 139, 168), // 1: Red
        (166, 227, 161), // 2: Green
        (249, 226, 175), // 3: Yellow
        (137, 180, 250), // 4: Blue
        (245, 194, 231), // 5: Magenta (Pink)
        (148, 226, 213), // 6: Cyan (Teal)
        (186, 194, 222), // 7: White (Subtext1)
        (88, 91, 112),   // 8: Bright Black (Surface2)
        (243, 139, 168), // 9: Bright Red
        (166, 227, 161), // 10: Bright Green
        (249, 226, 175), // 11: Bright Yellow
        (137, 180, 250), // 12: Bright Blue
        (245, 194, 231), // 13: Bright Magenta
        (148, 226, 213), // 14: Bright Cyan
        (166, 173, 200), // 15: Bright White (Subtext0)
    ],
};

pub static CATPPUCCIN_LATTE: Palette = Palette {
    name: "catppuccin-latte",
    fg: (76, 79, 105),       // #4c4f69
    bg: (239, 241, 245),     // #eff1f5
    cursor: (220, 138, 120), // #dc8a78
    ansi: [
        (92, 95, 119),   // 0: Black
        (210, 15, 57),   // 1: Red
        (64, 160, 43),   // 2: Green
        (223, 142, 29),  // 3: Yellow
        (30, 102, 245),  // 4: Blue
        (234, 118, 203), // 5: Magenta
        (23, 146, 153),  // 6: Cyan
        (172, 176, 190), // 7: White
        (108, 111, 133), // 8: Bright Black
        (210, 15, 57),   // 9: Bright Red
        (64, 160, 43),   // 10: Bright Green
        (223, 142, 29),  // 11: Bright Yellow
        (30, 102, 245),  // 12: Bright Blue
        (234, 118, 203), // 13: Bright Magenta
        (23, 146, 153),  // 14: Bright Cyan
        (188, 192, 204), // 15: Bright White
    ],
};

pub static DRACULA: Palette = Palette {
    name: "dracula",
    fg: (248, 248, 242),     // #f8f8f2
    bg: (40, 42, 54),        // #282a36
    cursor: (255, 121, 198), // #ff79c6
    ansi: [
        (33, 34, 44),    // 0: Black
        (255, 85, 85),   // 1: Red
        (80, 250, 123),  // 2: Green
        (241, 250, 140), // 3: Yellow
        (189, 147, 249), // 4: Blue / Purple
        (255, 121, 198), // 5: Magenta / Pink
        (139, 233, 253), // 6: Cyan
        (248, 248, 242), // 7: White
        (98, 114, 164),  // 8: Bright Black (Comment)
        (255, 110, 110), // 9: Bright Red
        (105, 255, 148), // 10: Bright Green
        (255, 255, 165), // 11: Bright Yellow
        (214, 172, 255), // 12: Bright Blue
        (255, 146, 223), // 13: Bright Magenta
        (164, 255, 255), // 14: Bright Cyan
        (255, 255, 255), // 15: Bright White
    ],
};

pub static NORD: Palette = Palette {
    name: "nord",
    fg: (216, 222, 233),     // #d8dee9
    bg: (46, 52, 64),        // #2e3440
    cursor: (136, 192, 208), // #88c0d0
    ansi: [
        (59, 66, 82),    // 0: nord1
        (191, 97, 106),  // 1: nord11 Red
        (163, 190, 140), // 2: nord14 Green
        (235, 203, 139), // 3: nord13 Yellow
        (129, 161, 193), // 4: nord9 Blue
        (180, 142, 173), // 5: nord15 Magenta
        (136, 192, 208), // 6: nord8 Cyan
        (229, 233, 240), // 7: nord5 White
        (76, 86, 106),   // 8: nord3 Bright Black
        (191, 97, 106),  // 9: Bright Red
        (163, 190, 140), // 10: Bright Green
        (235, 203, 139), // 11: Bright Yellow
        (129, 161, 193), // 12: Bright Blue
        (180, 142, 173), // 13: Bright Magenta
        (143, 188, 187), // 14: Bright Cyan (nord7)
        (236, 239, 244), // 15: Bright White (nord6)
    ],
};

pub static TOKYO_NIGHT: Palette = Palette {
    name: "tokyo-night",
    fg: (192, 202, 245),     // #c0caf5
    bg: (26, 27, 38),        // #1a1b26
    cursor: (192, 202, 245), // #c0caf5
    ansi: [
        (21, 22, 30),    // 0: Black
        (247, 118, 142), // 1: Red
        (158, 206, 106), // 2: Green
        (224, 175, 104), // 3: Yellow
        (122, 162, 247), // 4: Blue
        (187, 154, 247), // 5: Magenta
        (125, 207, 255), // 6: Cyan
        (169, 177, 214), // 7: White
        (65, 72, 104),   // 8: Bright Black
        (247, 118, 142), // 9: Bright Red
        (158, 206, 106), // 10: Bright Green
        (224, 175, 104), // 11: Bright Yellow
        (122, 162, 247), // 12: Bright Blue
        (187, 154, 247), // 13: Bright Magenta
        (125, 207, 255), // 14: Bright Cyan
        (192, 202, 245), // 15: Bright White
    ],
};

pub static GRUVBOX_DARK: Palette = Palette {
    name: "gruvbox-dark",
    fg: (235, 219, 178),     // #ebdbb2
    bg: (40, 40, 40),        // #282828
    cursor: (235, 219, 178), // #ebdbb2
    ansi: [
        (40, 40, 40),    // 0: Black
        (204, 36, 29),   // 1: Red
        (152, 151, 26),  // 2: Green
        (215, 153, 33),  // 3: Yellow
        (69, 133, 136),  // 4: Blue
        (177, 98, 134),  // 5: Magenta
        (104, 157, 106), // 6: Cyan
        (168, 153, 132), // 7: White
        (146, 131, 116), // 8: Bright Black
        (251, 73, 52),   // 9: Bright Red
        (184, 187, 38),  // 10: Bright Green
        (250, 189, 47),  // 11: Bright Yellow
        (131, 165, 152), // 12: Bright Blue
        (211, 134, 155), // 13: Bright Magenta
        (142, 192, 124), // 14: Bright Cyan
        (235, 219, 178), // 15: Bright White
    ],
};

pub static SOLARIZED_DARK: Palette = Palette {
    name: "solarized-dark",
    fg: (131, 148, 150),     // #839496
    bg: (0, 43, 54),         // #002b36
    cursor: (147, 161, 161), // #93a1a1
    ansi: [
        (7, 54, 66),     // 0: Base02
        (220, 50, 47),   // 1: Red
        (133, 153, 0),   // 2: Green
        (181, 137, 0),   // 3: Yellow
        (38, 139, 210),  // 4: Blue
        (211, 54, 130),  // 5: Magenta
        (42, 161, 152),  // 6: Cyan
        (238, 232, 213), // 7: Base2
        (0, 43, 54),     // 8: Base03
        (203, 75, 22),   // 9: Bright Red (Orange)
        (88, 110, 117),  // 10: Base01
        (101, 123, 131), // 11: Base00
        (131, 148, 150), // 12: Base0
        (108, 113, 196), // 13: Violet
        (147, 161, 161), // 14: Base1
        (253, 246, 227), // 15: Base3
    ],
};

pub static MONOKAI: Palette = Palette {
    name: "monokai",
    fg: (248, 248, 242),     // #f8f8f2
    bg: (39, 40, 34),        // #272822
    cursor: (248, 248, 240), // #f8f8f0
    ansi: [
        (39, 40, 34),    // 0: Black
        (249, 38, 114),  // 1: Red
        (166, 226, 46),  // 2: Green
        (244, 191, 117), // 3: Yellow
        (102, 217, 239), // 4: Blue
        (174, 129, 255), // 5: Magenta
        (161, 239, 228), // 6: Cyan
        (248, 248, 242), // 7: White
        (117, 113, 94),  // 8: Bright Black
        (249, 38, 114),  // 9: Bright Red
        (166, 226, 46),  // 10: Bright Green
        (244, 191, 117), // 11: Bright Yellow
        (102, 217, 239), // 12: Bright Blue
        (174, 129, 255), // 13: Bright Magenta
        (161, 239, 228), // 14: Bright Cyan
        (249, 248, 245), // 15: Bright White
    ],
};

pub static ONE_DARK: Palette = Palette {
    name: "one-dark",
    fg: (171, 178, 191),    // #abb2bf
    bg: (40, 44, 52),       // #282c34
    cursor: (82, 139, 255), // #528bff
    ansi: [
        (40, 44, 52),    // 0: Black
        (224, 108, 117), // 1: Red
        (152, 195, 121), // 2: Green
        (229, 192, 123), // 3: Yellow
        (97, 175, 239),  // 4: Blue
        (198, 120, 221), // 5: Magenta
        (86, 182, 194),  // 6: Cyan
        (171, 178, 191), // 7: White
        (92, 99, 112),   // 8: Bright Black
        (224, 108, 117), // 9: Bright Red
        (152, 195, 121), // 10: Bright Green
        (229, 192, 123), // 11: Bright Yellow
        (97, 175, 239),  // 12: Bright Blue
        (198, 120, 221), // 13: Bright Magenta
        (86, 182, 194),  // 14: Bright Cyan
        (255, 255, 255), // 15: Bright White
    ],
};

pub fn find_theme(name: &str) -> Option<&'static Palette> {
    let lower = name.to_ascii_lowercase().replace('_', "-");
    match lower.as_str() {
        "catppuccin-mocha" | "catppuccin" | "mocha" => Some(&CATPPUCCIN_MOCHA),
        "catppuccin-latte" | "latte" => Some(&CATPPUCCIN_LATTE),
        "dracula" => Some(&DRACULA),
        "nord" => Some(&NORD),
        "tokyo-night" | "tokyonight" => Some(&TOKYO_NIGHT),
        "gruvbox-dark" | "gruvbox" => Some(&GRUVBOX_DARK),
        "solarized-dark" | "solarized" => Some(&SOLARIZED_DARK),
        "monokai" => Some(&MONOKAI),
        "one-dark" | "onedark" => Some(&ONE_DARK),
        _ => None,
    }
}

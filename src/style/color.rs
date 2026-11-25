#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn hex(color: &str) -> Self {
        let stripped = match color.as_bytes() {
            [b'#', rest @ ..] => rest,
            bytes => bytes,
        };

        let [r, g, b, a] = match stripped.len() {
            3 => match const_hex::const_decode_to_array::<3>(stripped) {
                Ok([r, g, b]) => [r, g, b, 255],
                Err(_) => panic!("Invalid hex color string"),
            },

            6 => match const_hex::const_decode_to_array::<3>(stripped) {
                Ok([r, g, b]) => [r, g, b, 255],
                Err(_) => panic!("Invalid hex color string"),
            },

            8 => match const_hex::const_decode_to_array::<4>(stripped) {
                Ok(rgba) => rgba,
                Err(_) => panic!("Invalid hex color string"),
            },

            _ => panic!("Hex color must be 3 or 6 or 8 characters long"),
        };

        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    pub const fn opacity(mut self, a: f32) -> Self {
        self.a = a;
        self
    }

    pub fn to_rgba8(self) -> [u8; 4] {
        [
            (self.r.clamp(0.0, 1.0) * 255.0) as u8,
            (self.g.clamp(0.0, 1.0) * 255.0) as u8,
            (self.b.clamp(0.0, 1.0) * 255.0) as u8,
            (self.a.clamp(0.0, 1.0) * 255.0) as u8,
        ]
    }

    pub fn lerp(self, target: Color, dt: f32) -> Color {
        let t = dt.clamp(0.0, 1.0);
        Color::rgba(
            self.r + (target.r - self.r) * t,
            self.g + (target.g - self.g) * t,
            self.b + (target.b - self.b) * t,
            self.a + (target.a - self.a) * t,
        )
    }

    pub const fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Color {
    pub const TRANSPARENT: Color = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Color = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Self::rgba(0.0, 0.0, 0.0, 1.0);

    pub const NEUTRAL_50: Color = Self::hex("#fafafa");
    pub const NEUTRAL_100: Color = Self::hex("#f5f5f5");
    pub const NEUTRAL_200: Color = Self::hex("#e5e5e5");
    pub const NEUTRAL_300: Color = Self::hex("#d4d4d4");
    pub const NEUTRAL_400: Color = Self::hex("#a3a3a3");
    pub const NEUTRAL_500: Color = Self::hex("#737373");
    pub const NEUTRAL_600: Color = Self::hex("#525252");
    pub const NEUTRAL_700: Color = Self::hex("#404040");
    pub const NEUTRAL_800: Color = Self::hex("#262626");
    pub const NEUTRAL_900: Color = Self::hex("#171717");
    pub const NEUTRAL_950: Color = Self::hex("#0a0a0a");

    pub const STONE_50: Color = Self::hex("#fafaf9");
    pub const STONE_100: Color = Self::hex("#f5f5f4");
    pub const STONE_200: Color = Self::hex("#e7e5e4");
    pub const STONE_300: Color = Self::hex("#d6d3d1");
    pub const STONE_400: Color = Self::hex("#a8a29e");
    pub const STONE_500: Color = Self::hex("#78716c");
    pub const STONE_600: Color = Self::hex("#57534e");
    pub const STONE_700: Color = Self::hex("#44403c");
    pub const STONE_800: Color = Self::hex("#292524");
    pub const STONE_900: Color = Self::hex("#1c1917");
    pub const STONE_950: Color = Self::hex("#0c0a09");

    pub const ZINC_50: Color = Self::hex("#fafafa");
    pub const ZINC_100: Color = Self::hex("#f4f4f5");
    pub const ZINC_200: Color = Self::hex("#e4e4e7");
    pub const ZINC_300: Color = Self::hex("#d4d4d8");
    pub const ZINC_400: Color = Self::hex("#a1a1aa");
    pub const ZINC_500: Color = Self::hex("#71717a");
    pub const ZINC_600: Color = Self::hex("#52525b");
    pub const ZINC_700: Color = Self::hex("#3f3f46");
    pub const ZINC_800: Color = Self::hex("#27272a");
    pub const ZINC_900: Color = Self::hex("#18181b");
    pub const ZINC_950: Color = Self::hex("#09090b");

    pub const SLATE_50: Color = Self::hex("#f8fafc");
    pub const SLATE_100: Color = Self::hex("#f1f5f9");
    pub const SLATE_200: Color = Self::hex("#e2e8f0");
    pub const SLATE_300: Color = Self::hex("#cbd5e1");
    pub const SLATE_400: Color = Self::hex("#94a3b8");
    pub const SLATE_500: Color = Self::hex("#64748b");
    pub const SLATE_600: Color = Self::hex("#475569");
    pub const SLATE_700: Color = Self::hex("#334155");
    pub const SLATE_800: Color = Self::hex("#1e293b");
    pub const SLATE_900: Color = Self::hex("#0f172a");
    pub const SLATE_950: Color = Self::hex("#020617");

    pub const GRAY_50: Color = Self::hex("#f9fafb");
    pub const GRAY_100: Color = Self::hex("#f3f4f6");
    pub const GRAY_200: Color = Self::hex("#e5e7eb");
    pub const GRAY_300: Color = Self::hex("#d1d5db");
    pub const GRAY_400: Color = Self::hex("#9ca3af");
    pub const GRAY_500: Color = Self::hex("#6b7280");
    pub const GRAY_600: Color = Self::hex("#4b5563");
    pub const GRAY_700: Color = Self::hex("#374151");
    pub const GRAY_800: Color = Self::hex("#1f2937");
    pub const GRAY_900: Color = Self::hex("#111827");
    pub const GRAY_950: Color = Self::hex("#030712");

    pub const MAUVE_50: Color = Self::hex("#fafafb");
    pub const MAUVE_100: Color = Self::hex("#f4f2f6");
    pub const MAUVE_200: Color = Self::hex("#e5e1e8");
    pub const MAUVE_300: Color = Self::hex("#d0c9d5");
    pub const MAUVE_400: Color = Self::hex("#a69bae");
    pub const MAUVE_500: Color = Self::hex("#81758b");
    pub const MAUVE_600: Color = Self::hex("#675d70");
    pub const MAUVE_700: Color = Self::hex("#524959");
    pub const MAUVE_800: Color = Self::hex("#3d3642");
    pub const MAUVE_900: Color = Self::hex("#2a252e");
    pub const MAUVE_950: Color = Self::hex("#161218");

    pub const OLIVE_50: Color = Self::hex("#f9faf9");
    pub const OLIVE_100: Color = Self::hex("#f1f4f2");
    pub const OLIVE_200: Color = Self::hex("#dee5e0");
    pub const OLIVE_300: Color = Self::hex("#c4cfc8");
    pub const OLIVE_400: Color = Self::hex("#94a599");
    pub const OLIVE_500: Color = Self::hex("#6c8072");
    pub const OLIVE_600: Color = Self::hex("#56675b");
    pub const OLIVE_700: Color = Self::hex("#435147");
    pub const OLIVE_800: Color = Self::hex("#313c35");
    pub const OLIVE_900: Color = Self::hex("#222a24");
    pub const OLIVE_950: Color = Self::hex("#101512");

    pub const MIST_50: Color = Self::hex("#f9fafb");
    pub const MIST_100: Color = Self::hex("#f0f4f5");
    pub const MIST_200: Color = Self::hex("#dce5e6");
    pub const MIST_300: Color = Self::hex("#c1cfd2");
    pub const MIST_400: Color = Self::hex("#8da5a9");
    pub const MIST_500: Color = Self::hex("#648085");
    pub const MIST_600: Color = Self::hex("#4f676b");
    pub const MIST_700: Color = Self::hex("#3d5155");
    pub const MIST_800: Color = Self::hex("#2d3c3f");
    pub const MIST_900: Color = Self::hex("#1e2a2c");
    pub const MIST_950: Color = Self::hex("#0e1517");

    pub const TAUPE_50: Color = Self::hex("#fbfaf9");
    pub const TAUPE_100: Color = Self::hex("#f5f3f0");
    pub const TAUPE_200: Color = Self::hex("#e7e2dc");
    pub const TAUPE_300: Color = Self::hex("#d3cbc0");
    pub const TAUPE_400: Color = Self::hex("#aa9e8d");
    pub const TAUPE_500: Color = Self::hex("#867865");
    pub const TAUPE_600: Color = Self::hex("#6c6050");
    pub const TAUPE_700: Color = Self::hex("#554b3e");
    pub const TAUPE_800: Color = Self::hex("#40382d");
    pub const TAUPE_900: Color = Self::hex("#2c271f");
    pub const TAUPE_950: Color = Self::hex("#17130e");

    pub const RED_50: Color = Self::hex("#fef2f2");
    pub const RED_100: Color = Self::hex("#fee2e2");
    pub const RED_200: Color = Self::hex("#fecaca");
    pub const RED_300: Color = Self::hex("#fca5a5");
    pub const RED_400: Color = Self::hex("#f87171");
    pub const RED_500: Color = Self::hex("#ef4444");
    pub const RED_600: Color = Self::hex("#dc2626");
    pub const RED_700: Color = Self::hex("#b91c1c");
    pub const RED_800: Color = Self::hex("#991b1b");
    pub const RED_900: Color = Self::hex("#7f1d1d");
    pub const RED_950: Color = Self::hex("#450a0a");

    pub const ORANGE_50: Color = Self::hex("#fff7ed");
    pub const ORANGE_100: Color = Self::hex("#ffedd5");
    pub const ORANGE_200: Color = Self::hex("#fed7aa");
    pub const ORANGE_300: Color = Self::hex("#fdba74");
    pub const ORANGE_400: Color = Self::hex("#fb923c");
    pub const ORANGE_500: Color = Self::hex("#f97316");
    pub const ORANGE_600: Color = Self::hex("#ea580c");
    pub const ORANGE_700: Color = Self::hex("#c2410c");
    pub const ORANGE_800: Color = Self::hex("#9a3412");
    pub const ORANGE_900: Color = Self::hex("#7c2d12");
    pub const ORANGE_950: Color = Self::hex("#431407");

    pub const AMBER_50: Color = Self::hex("#fffbeb");
    pub const AMBER_100: Color = Self::hex("#fef3c7");
    pub const AMBER_200: Color = Self::hex("#fde68a");
    pub const AMBER_300: Color = Self::hex("#fcd34d");
    pub const AMBER_400: Color = Self::hex("#fbbf24");
    pub const AMBER_500: Color = Self::hex("#f59e0b");
    pub const AMBER_600: Color = Self::hex("#d97706");
    pub const AMBER_700: Color = Self::hex("#b45309");
    pub const AMBER_800: Color = Self::hex("#92400e");
    pub const AMBER_900: Color = Self::hex("#78350f");
    pub const AMBER_950: Color = Self::hex("#451a03");

    pub const YELLOW_50: Color = Self::hex("#fefce8");
    pub const YELLOW_100: Color = Self::hex("#fef9c3");
    pub const YELLOW_200: Color = Self::hex("#fef08a");
    pub const YELLOW_300: Color = Self::hex("#fde047");
    pub const YELLOW_400: Color = Self::hex("#facc15");
    pub const YELLOW_500: Color = Self::hex("#eab308");
    pub const YELLOW_600: Color = Self::hex("#ca8a04");
    pub const YELLOW_700: Color = Self::hex("#a16207");
    pub const YELLOW_800: Color = Self::hex("#854d0e");
    pub const YELLOW_900: Color = Self::hex("#713f12");
    pub const YELLOW_950: Color = Self::hex("#422006");

    pub const LIME_50: Color = Self::hex("#f7fee7");
    pub const LIME_100: Color = Self::hex("#ecfccb");
    pub const LIME_200: Color = Self::hex("#d9f99d");
    pub const LIME_300: Color = Self::hex("#bef264");
    pub const LIME_400: Color = Self::hex("#a3e635");
    pub const LIME_500: Color = Self::hex("#84cc16");
    pub const LIME_600: Color = Self::hex("#65a30d");
    pub const LIME_700: Color = Self::hex("#4d7c0f");
    pub const LIME_800: Color = Self::hex("#3f6212");
    pub const LIME_900: Color = Self::hex("#365314");
    pub const LIME_950: Color = Self::hex("#1a2e05");

    pub const GREEN_50: Color = Self::hex("#f0fdf4");
    pub const GREEN_100: Color = Self::hex("#dcfce7");
    pub const GREEN_200: Color = Self::hex("#bbf7d0");
    pub const GREEN_300: Color = Self::hex("#86efac");
    pub const GREEN_400: Color = Self::hex("#4ade80");
    pub const GREEN_500: Color = Self::hex("#22c55e");
    pub const GREEN_600: Color = Self::hex("#16a34a");
    pub const GREEN_700: Color = Self::hex("#15803d");
    pub const GREEN_800: Color = Self::hex("#166534");
    pub const GREEN_900: Color = Self::hex("#14532d");
    pub const GREEN_950: Color = Self::hex("#052e16");

    pub const EMERALD_50: Color = Self::hex("#ecfdf5");
    pub const EMERALD_100: Color = Self::hex("#d1fae5");
    pub const EMERALD_200: Color = Self::hex("#a7f3d0");
    pub const EMERALD_300: Color = Self::hex("#6ee7b7");
    pub const EMERALD_400: Color = Self::hex("#34d399");
    pub const EMERALD_500: Color = Self::hex("#10b981");
    pub const EMERALD_600: Color = Self::hex("#059669");
    pub const EMERALD_700: Color = Self::hex("#047857");
    pub const EMERALD_800: Color = Self::hex("#065f46");
    pub const EMERALD_900: Color = Self::hex("#064e3b");
    pub const EMERALD_950: Color = Self::hex("#022c22");

    pub const TEAL_50: Color = Self::hex("#f0fdfa");
    pub const TEAL_100: Color = Self::hex("#ccfbf1");
    pub const TEAL_200: Color = Self::hex("#99f6e4");
    pub const TEAL_300: Color = Self::hex("#5eead4");
    pub const TEAL_400: Color = Self::hex("#2dd4bf");
    pub const TEAL_500: Color = Self::hex("#14b8a6");
    pub const TEAL_600: Color = Self::hex("#0d9488");
    pub const TEAL_700: Color = Self::hex("#0f766e");
    pub const TEAL_800: Color = Self::hex("#115e59");
    pub const TEAL_900: Color = Self::hex("#134e4a");
    pub const TEAL_950: Color = Self::hex("#042f2e");

    pub const CYAN_50: Color = Self::hex("#ecfeff");
    pub const CYAN_100: Color = Self::hex("#cffafe");
    pub const CYAN_200: Color = Self::hex("#a5f3fc");
    pub const CYAN_300: Color = Self::hex("#67e8f9");
    pub const CYAN_400: Color = Self::hex("#22d3ee");
    pub const CYAN_500: Color = Self::hex("#06b6d4");
    pub const CYAN_600: Color = Self::hex("#0891b2");
    pub const CYAN_700: Color = Self::hex("#0e7490");
    pub const CYAN_800: Color = Self::hex("#155e75");
    pub const CYAN_900: Color = Self::hex("#164e63");
    pub const CYAN_950: Color = Self::hex("#083344");

    pub const SKY_50: Color = Self::hex("#f0f9ff");
    pub const SKY_100: Color = Self::hex("#e0f2fe");
    pub const SKY_200: Color = Self::hex("#bae6fd");
    pub const SKY_300: Color = Self::hex("#7dd3fc");
    pub const SKY_400: Color = Self::hex("#38bdf8");
    pub const SKY_500: Color = Self::hex("#0ea5e9");
    pub const SKY_600: Color = Self::hex("#0284c7");
    pub const SKY_700: Color = Self::hex("#0369a1");
    pub const SKY_800: Color = Self::hex("#075985");
    pub const SKY_900: Color = Self::hex("#0c4a6e");
    pub const SKY_950: Color = Self::hex("#082f49");

    pub const BLUE_50: Color = Self::hex("#eff6ff");
    pub const BLUE_100: Color = Self::hex("#dbeafe");
    pub const BLUE_200: Color = Self::hex("#bfdbfe");
    pub const BLUE_300: Color = Self::hex("#93c5fd");
    pub const BLUE_400: Color = Self::hex("#60a5fa");
    pub const BLUE_500: Color = Self::hex("#3b82f6");
    pub const BLUE_600: Color = Self::hex("#2563eb");
    pub const BLUE_700: Color = Self::hex("#1d4ed8");
    pub const BLUE_800: Color = Self::hex("#1e40af");
    pub const BLUE_900: Color = Self::hex("#1e3a8a");
    pub const BLUE_950: Color = Self::hex("#172554");

    pub const INDIGO_50: Color = Self::hex("#eef2ff");
    pub const INDIGO_100: Color = Self::hex("#e0e7ff");
    pub const INDIGO_200: Color = Self::hex("#c7d2fe");
    pub const INDIGO_300: Color = Self::hex("#a5b4fc");
    pub const INDIGO_400: Color = Self::hex("#818cf8");
    pub const INDIGO_500: Color = Self::hex("#6366f1");
    pub const INDIGO_600: Color = Self::hex("#4f46e5");
    pub const INDIGO_700: Color = Self::hex("#4338ca");
    pub const INDIGO_800: Color = Self::hex("#3730a3");
    pub const INDIGO_900: Color = Self::hex("#312e81");
    pub const INDIGO_950: Color = Self::hex("#1e1b4b");

    pub const VIOLET_50: Color = Self::hex("#f5f3ff");
    pub const VIOLET_100: Color = Self::hex("#ede9fe");
    pub const VIOLET_200: Color = Self::hex("#ddd6fe");
    pub const VIOLET_300: Color = Self::hex("#c4b5fd");
    pub const VIOLET_400: Color = Self::hex("#a78bfa");
    pub const VIOLET_500: Color = Self::hex("#8b5cf6");
    pub const VIOLET_600: Color = Self::hex("#7c3aed");
    pub const VIOLET_700: Color = Self::hex("#6d28d9");
    pub const VIOLET_800: Color = Self::hex("#5b21b6");
    pub const VIOLET_900: Color = Self::hex("#4c1d95");
    pub const VIOLET_950: Color = Self::hex("#2e1065");

    pub const PURPLE_50: Color = Self::hex("#faf5ff");
    pub const PURPLE_100: Color = Self::hex("#f3e8ff");
    pub const PURPLE_200: Color = Self::hex("#e9d5ff");
    pub const PURPLE_300: Color = Self::hex("#d8b4fe");
    pub const PURPLE_400: Color = Self::hex("#c084fc");
    pub const PURPLE_500: Color = Self::hex("#a855f7");
    pub const PURPLE_600: Color = Self::hex("#9333ea");
    pub const PURPLE_700: Color = Self::hex("#7e22ce");
    pub const PURPLE_800: Color = Self::hex("#6b21a8");
    pub const PURPLE_900: Color = Self::hex("#581c87");
    pub const PURPLE_950: Color = Self::hex("#3b0764");

    pub const FUCHSIA_50: Color = Self::hex("#fdf4ff");
    pub const FUCHSIA_100: Color = Self::hex("#fae8ff");
    pub const FUCHSIA_200: Color = Self::hex("#f5d0fe");
    pub const FUCHSIA_300: Color = Self::hex("#f0abfc");
    pub const FUCHSIA_400: Color = Self::hex("#e879f9");
    pub const FUCHSIA_500: Color = Self::hex("#d946ef");
    pub const FUCHSIA_600: Color = Self::hex("#c026d3");
    pub const FUCHSIA_700: Color = Self::hex("#a21caf");
    pub const FUCHSIA_800: Color = Self::hex("#86198f");
    pub const FUCHSIA_900: Color = Self::hex("#701a75");
    pub const FUCHSIA_950: Color = Self::hex("#4a044e");

    pub const PINK_50: Color = Self::hex("#fdf2f8");
    pub const PINK_100: Color = Self::hex("#fce7f3");
    pub const PINK_200: Color = Self::hex("#fbcfe8");
    pub const PINK_300: Color = Self::hex("#f9a8d4");
    pub const PINK_400: Color = Self::hex("#f472b6");
    pub const PINK_500: Color = Self::hex("#ec4899");
    pub const PINK_600: Color = Self::hex("#db2777");
    pub const PINK_700: Color = Self::hex("#be185d");
    pub const PINK_800: Color = Self::hex("#9d174d");
    pub const PINK_900: Color = Self::hex("#831843");
    pub const PINK_950: Color = Self::hex("#500724");

    pub const ROSE_50: Color = Self::hex("#fff1f2");
    pub const ROSE_100: Color = Self::hex("#ffe4e6");
    pub const ROSE_200: Color = Self::hex("#fecdd3");
    pub const ROSE_300: Color = Self::hex("#fda4af");
    pub const ROSE_400: Color = Self::hex("#fb7185");
    pub const ROSE_500: Color = Self::hex("#f43f5e");
    pub const ROSE_600: Color = Self::hex("#e11d48");
    pub const ROSE_700: Color = Self::hex("#be123c");
    pub const ROSE_800: Color = Self::hex("#9f1239");
    pub const ROSE_900: Color = Self::hex("#881337");
    pub const ROSE_950: Color = Self::hex("#4c0519");
}

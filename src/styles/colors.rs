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

    pub const fn hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self::rgba(r, g, b, 1.0)
    }
}

impl Color {
    pub const TRANSPARENT: Color = Self::rgba(0.0, 0.0, 0.0, 0.0);
    pub const WHITE: Color = Self::rgba(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Color = Self::rgba(0.0, 0.0, 0.0, 1.0);

    pub const RED_50: Color = Self::hex(0xfef2f2);
    pub const RED_100: Color = Self::hex(0xffe2e2);
    pub const RED_200: Color = Self::hex(0xffc9c9);
    pub const RED_300: Color = Self::hex(0xffa2a2);
    pub const RED_400: Color = Self::hex(0xff6467);
    pub const RED_500: Color = Self::hex(0xfb2c36);
    pub const RED_600: Color = Self::hex(0xe7000b);
    pub const RED_700: Color = Self::hex(0xc10007);
    pub const RED_800: Color = Self::hex(0x9f0712);
    pub const RED_900: Color = Self::hex(0x82181a);
    pub const RED_950: Color = Self::hex(0x460809);

    pub const ORANGE_50: Color = Self::hex(0xfff7ed);
    pub const ORANGE_100: Color = Self::hex(0xffedd4);
    pub const ORANGE_200: Color = Self::hex(0xffd6a7);
    pub const ORANGE_300: Color = Self::hex(0xffb86a);
    pub const ORANGE_400: Color = Self::hex(0xff8904);
    pub const ORANGE_500: Color = Self::hex(0xff6900);
    pub const ORANGE_600: Color = Self::hex(0xf54900);
    pub const ORANGE_700: Color = Self::hex(0xca3500);
    pub const ORANGE_800: Color = Self::hex(0x9f2d00);
    pub const ORANGE_900: Color = Self::hex(0x7e2a0c);
    pub const ORANGE_950: Color = Self::hex(0x441306);

    pub const AMBER_50: Color = Self::hex(0xfffbeb);
    pub const AMBER_100: Color = Self::hex(0xfef3c6);
    pub const AMBER_200: Color = Self::hex(0xfee685);
    pub const AMBER_300: Color = Self::hex(0xffd230);
    pub const AMBER_400: Color = Self::hex(0xffba00);
    pub const AMBER_500: Color = Self::hex(0xfd9a00);
    pub const AMBER_600: Color = Self::hex(0xe17100);
    pub const AMBER_700: Color = Self::hex(0xbb4d00);
    pub const AMBER_800: Color = Self::hex(0x973c00);
    pub const AMBER_900: Color = Self::hex(0x7b3306);
    pub const AMBER_950: Color = Self::hex(0x461901);

    pub const YELLOW_50: Color = Self::hex(0xfefce8);
    pub const YELLOW_100: Color = Self::hex(0xfef9c2);
    pub const YELLOW_200: Color = Self::hex(0xfff085);
    pub const YELLOW_300: Color = Self::hex(0xffdf20);
    pub const YELLOW_400: Color = Self::hex(0xfcc800);
    pub const YELLOW_500: Color = Self::hex(0xefb100);
    pub const YELLOW_600: Color = Self::hex(0xd08700);
    pub const YELLOW_700: Color = Self::hex(0xa65f00);
    pub const YELLOW_800: Color = Self::hex(0x894b00);
    pub const YELLOW_900: Color = Self::hex(0x733e0a);
    pub const YELLOW_950: Color = Self::hex(0x432004);

    pub const LIME_50: Color = Self::hex(0xf7fee7);
    pub const LIME_100: Color = Self::hex(0xecfcca);
    pub const LIME_200: Color = Self::hex(0xd8f999);
    pub const LIME_300: Color = Self::hex(0xbbf451);
    pub const LIME_400: Color = Self::hex(0x9ae600);
    pub const LIME_500: Color = Self::hex(0x7ccf00);
    pub const LIME_600: Color = Self::hex(0x5ea500);
    pub const LIME_700: Color = Self::hex(0x497d00);
    pub const LIME_800: Color = Self::hex(0x3c6300);
    pub const LIME_900: Color = Self::hex(0x35530e);
    pub const LIME_950: Color = Self::hex(0x192e03);

    pub const GREEN_50: Color = Self::hex(0xf0fdf4);
    pub const GREEN_100: Color = Self::hex(0xdcfce7);
    pub const GREEN_200: Color = Self::hex(0xb9f8cf);
    pub const GREEN_300: Color = Self::hex(0x7bf1a8);
    pub const GREEN_400: Color = Self::hex(0x05df72);
    pub const GREEN_500: Color = Self::hex(0x00c950);
    pub const GREEN_600: Color = Self::hex(0x00a63e);
    pub const GREEN_700: Color = Self::hex(0x008236);
    pub const GREEN_800: Color = Self::hex(0x016630);
    pub const GREEN_900: Color = Self::hex(0x0d542b);
    pub const GREEN_950: Color = Self::hex(0x032e15);

    pub const EMERALD_50: Color = Self::hex(0xecfdf5);
    pub const EMERALD_100: Color = Self::hex(0xd0fae5);
    pub const EMERALD_200: Color = Self::hex(0xa4f4cf);
    pub const EMERALD_300: Color = Self::hex(0x5ee9b5);
    pub const EMERALD_400: Color = Self::hex(0x00d492);
    pub const EMERALD_500: Color = Self::hex(0x00bc7d);
    pub const EMERALD_600: Color = Self::hex(0x009966);
    pub const EMERALD_700: Color = Self::hex(0x007a55);
    pub const EMERALD_800: Color = Self::hex(0x006045);
    pub const EMERALD_900: Color = Self::hex(0x004f3b);
    pub const EMERALD_950: Color = Self::hex(0x002c22);

    pub const TEAL_50: Color = Self::hex(0xf0fdfa);
    pub const TEAL_100: Color = Self::hex(0xcbfbf1);
    pub const TEAL_200: Color = Self::hex(0x96f7e4);
    pub const TEAL_300: Color = Self::hex(0x46ecd5);
    pub const TEAL_400: Color = Self::hex(0x00d5be);
    pub const TEAL_500: Color = Self::hex(0x00bba7);
    pub const TEAL_600: Color = Self::hex(0x009689);
    pub const TEAL_700: Color = Self::hex(0x00786f);
    pub const TEAL_800: Color = Self::hex(0x005f5a);
    pub const TEAL_900: Color = Self::hex(0x0b4f4a);
    pub const TEAL_950: Color = Self::hex(0x022f2e);

    pub const CYAN_50: Color = Self::hex(0xecfeff);
    pub const CYAN_100: Color = Self::hex(0xcefafe);
    pub const CYAN_200: Color = Self::hex(0xa2f4fd);
    pub const CYAN_300: Color = Self::hex(0x53eafd);
    pub const CYAN_400: Color = Self::hex(0x00d3f2);
    pub const CYAN_500: Color = Self::hex(0x00b8db);
    pub const CYAN_600: Color = Self::hex(0x0092b8);
    pub const CYAN_700: Color = Self::hex(0x007595);
    pub const CYAN_800: Color = Self::hex(0x005f78);
    pub const CYAN_900: Color = Self::hex(0x104e64);
    pub const CYAN_950: Color = Self::hex(0x053345);

    pub const SKY_50: Color = Self::hex(0xf0f9ff);
    pub const SKY_100: Color = Self::hex(0xdff2fe);
    pub const SKY_200: Color = Self::hex(0xb8e6fe);
    pub const SKY_300: Color = Self::hex(0x74d4ff);
    pub const SKY_400: Color = Self::hex(0x00bcff);
    pub const SKY_500: Color = Self::hex(0x00a6f4);
    pub const SKY_600: Color = Self::hex(0x0084d1);
    pub const SKY_700: Color = Self::hex(0x0069a8);
    pub const SKY_800: Color = Self::hex(0x00598a);
    pub const SKY_900: Color = Self::hex(0x024a70);
    pub const SKY_950: Color = Self::hex(0x052f4a);

    pub const BLUE_50: Color = Self::hex(0xeff6ff);
    pub const BLUE_100: Color = Self::hex(0xdbeafe);
    pub const BLUE_200: Color = Self::hex(0xbedbff);
    pub const BLUE_300: Color = Self::hex(0x8ec5ff);
    pub const BLUE_400: Color = Self::hex(0x51a2ff);
    pub const BLUE_500: Color = Self::hex(0x2b7fff);
    pub const BLUE_600: Color = Self::hex(0x155dfc);
    pub const BLUE_700: Color = Self::hex(0x1447e6);
    pub const BLUE_800: Color = Self::hex(0x193cb8);
    pub const BLUE_900: Color = Self::hex(0x1c398e);
    pub const BLUE_950: Color = Self::hex(0x162456);

    pub const INDIGO_50: Color = Self::hex(0xeef2ff);
    pub const INDIGO_100: Color = Self::hex(0xe0e7ff);
    pub const INDIGO_200: Color = Self::hex(0xc6d2ff);
    pub const INDIGO_300: Color = Self::hex(0xa3b3ff);
    pub const INDIGO_400: Color = Self::hex(0x7c86ff);
    pub const INDIGO_500: Color = Self::hex(0x615fff);
    pub const INDIGO_600: Color = Self::hex(0x4f39f6);
    pub const INDIGO_700: Color = Self::hex(0x432dd7);
    pub const INDIGO_800: Color = Self::hex(0x372aac);
    pub const INDIGO_900: Color = Self::hex(0x312c85);
    pub const INDIGO_950: Color = Self::hex(0x1e1a4d);
}

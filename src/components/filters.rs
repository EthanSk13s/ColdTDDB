use iced::{
    Checkbox, Row, Element, Text,
    pick_list, PickList
};

use crate::app::Message;

#[derive(Debug, Clone)]
pub struct TypeFilter {
    pub princess_toggle: bool,
    pub fairy_toggle: bool,
    pub angel_toggle: bool,
    pub extra_toggle: bool
}

impl TypeFilter {
    pub fn new() -> Self {
        TypeFilter {
            princess_toggle: true,
            fairy_toggle: true,
            angel_toggle: true,
            extra_toggle: true
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let type_row = Row::new().push(Text::new("Idol type:"));

        let princess_toggle = Checkbox::new(
            self.princess_toggle,
            "Princess",
            move|toggle| {Message::ToggleType(toggle, 1)}
        )
        .spacing(5);

        let fairy_toggle = Checkbox::new(
            self.fairy_toggle,
            "Fairy",
            move|toggle| {Message::ToggleType(toggle, 2)}
        )
        .spacing(5);

        let angel_toggle = Checkbox::new(
            self.angel_toggle,
            "Angel",
            move|toggle| {Message::ToggleType(toggle, 3)}
        )
        .spacing(5);

        let extra_toggle = Checkbox::new(
            self.extra_toggle,
            "Extra",
            move|toggle| {Message::ToggleType(toggle, 5)}
        )
        .spacing(5);

        type_row
            .spacing(10)
            .push(princess_toggle)
            .push(fairy_toggle)
            .push(angel_toggle)
            .push(extra_toggle)
            .into()
    }

    pub fn set_state(&mut self, value: i32, state: bool) {
        match value {
            1 => self.princess_toggle = state,
            2 => self.fairy_toggle = state,
            3 => self.angel_toggle = state,
            5 => self.extra_toggle = state,
            _ => (),
        };
    }
}

#[derive(Debug, Clone)]
pub struct RarityFilter {
    pub n_toggle: bool,
    pub r_toggle: bool,
    pub sr_toggle: bool,
    pub ssr_toggle: bool
}

impl RarityFilter {
    pub fn new() -> Self {
        RarityFilter {
            n_toggle: true,
            r_toggle: true,
            sr_toggle: true,
            ssr_toggle: true
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let rarity_row = Row::new().push(Text::new("Rarity:"));

        let n_radio = Checkbox::new(
            self.n_toggle,
            "N",
            move|toggle| {Message::ToggleRarity(toggle, 1)}
        )
        .spacing(5);

        let r_radio = Checkbox::new(
            self.r_toggle,
            "R",
            move|toggle| {Message::ToggleRarity(toggle, 2)}
        )
        .spacing(5);

        let sr_radio = Checkbox::new(
            self.sr_toggle,
            "SR",
            move|toggle| {Message::ToggleRarity(toggle, 3)}
        )
        .spacing(5);

        let ssr_radio = Checkbox::new(
            self.ssr_toggle,
            "SSR",
            move|toggle| {Message::ToggleRarity(toggle, 4)}
        )
        .spacing(5);

        rarity_row
            .spacing(10)
            .push(n_radio)
            .push(r_radio)
            .push(sr_radio)
            .push(ssr_radio)
            .into()

    }

    pub fn set_state(&mut self, value: i32, state: bool) {
        match value {
            1 => self.n_toggle = state,
            2 => self.r_toggle = state,
            3 => self.sr_toggle = state,
            4 => self.ssr_toggle = state,
            _ => (),
        };
    }
}


// This has too many options, maybe need something different than a picklist
#[derive(Debug, Clone)]
pub struct IdolFilter {
    idol_list: pick_list::State<IdolList>,
    pub selected: IdolList
}

impl IdolFilter {
    pub fn new() -> Self {
        IdolFilter {
            idol_list: pick_list::State::default(),
            selected: IdolList::default()
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let row = Row::new().push(Text::new("Idol:"));

        let idols = PickList::new(
            &mut self.idol_list,
            &IdolList::ALL[..],
            Some(self.selected),
            Message::PickIdol
        );

        row.push(idols).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IdolList {
    All = 0,
    Haruka = 1,
    Chihaya = 2,
    Miki = 3,
    Yukiho = 4,
    Yayoi = 5,
    Makoto = 6,
    Iori = 7,
    Takane = 8,
    Ritsuko = 9,
    Azusa = 10,
    Ami = 11,
    Mami = 12,
    Hibiki = 13,
    Mirai = 14,
    Shizuka = 15,
    Tsubasa = 16,
    Kotoha = 17,
    Elena = 18,
    Minako = 19,
    Megumi = 20,
    Matsuri = 21,
    Serika = 22,
    Akane = 23,
    Anna = 24,
    Roco = 25,
    Yuriko = 26,
    Sayoko = 27,
    Arisa = 28,
    Umi = 29,
    Iku = 30,
    Tomoka = 31,
    Emily = 32,
    Shiho = 33,
    Ayumu = 34,
    Hinata = 35,
    Kana = 36,
    Nao = 37,
    Chizuru = 38,
    Konomi = 39,
    Tamaki = 40,
    Fuka = 41,
    Miya = 42,
    Noriko = 43,
    Mizuki = 44,
    Karen = 45,
    Rio = 46,
    Subaru = 47,
    Reika = 48,
    Momoko = 49,
    Julia = 50,
    Tsumugi = 51,
    Kaori = 52,
    Shika = 201,
    Leon = 202,
    Frederica = 204,
    Shiki = 205
}

impl IdolList {
    pub const ALL: [IdolList; 57] = [
        IdolList::All,
        IdolList::Haruka,
        IdolList::Chihaya,
        IdolList::Miki,
        IdolList::Yukiho,
        IdolList::Yayoi,
        IdolList::Makoto,
        IdolList::Iori,
        IdolList::Takane,
        IdolList::Ritsuko,
        IdolList::Azusa,
        IdolList::Ami,
        IdolList::Mami,
        IdolList::Hibiki,
        IdolList::Mirai,
        IdolList::Shizuka,
        IdolList::Tsubasa,
        IdolList::Kotoha,
        IdolList::Elena,
        IdolList::Minako,
        IdolList::Megumi,
        IdolList::Matsuri,
        IdolList::Serika,
        IdolList::Akane,
        IdolList::Anna,
        IdolList::Roco,
        IdolList::Yuriko,
        IdolList::Sayoko,
        IdolList::Arisa,
        IdolList::Umi,
        IdolList::Iku,
        IdolList::Tomoka,
        IdolList::Emily,
        IdolList::Shiho,
        IdolList::Ayumu,
        IdolList::Hinata,
        IdolList::Kana,
        IdolList::Nao,
        IdolList::Chizuru,
        IdolList::Konomi,
        IdolList::Tamaki,
        IdolList::Fuka,
        IdolList::Miya,
        IdolList::Noriko,
        IdolList::Mizuki,
        IdolList::Karen,
        IdolList::Rio,
        IdolList::Subaru,
        IdolList::Reika,
        IdolList::Momoko,
        IdolList::Julia,
        IdolList::Tsumugi,
        IdolList::Kaori,
        IdolList::Shika,
        IdolList::Leon,
        IdolList::Frederica,
        IdolList::Shiki
    ];
}

impl Default for IdolList {
    fn default() -> IdolList {
        IdolList::All
    }
}

impl std::fmt::Display for IdolList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdolList::All => write!(f, "----"),
            IdolList::Haruka => write!(f, "Amami Haruka"),
            IdolList::Chihaya => write!(f, "Kisaragi Chihaya"),
            IdolList::Miki => write!(f, "Hoshii Miki"),
            IdolList::Yukiho => write!(f, "Sugawara Yukiho"),
            IdolList::Yayoi => write!(f, "Takatsuki Yayoi"),
            IdolList::Makoto => write!(f, "Kikuchi Makoto"),
            IdolList::Iori => write!(f, "Minase Iori"),
            IdolList::Takane => write!(f, "Shijou Takane"),
            IdolList::Ritsuko => write!(f, "Akizuki Ritsuko"),
            IdolList::Azusa => write!(f, "Miura Azusa"),
            IdolList::Ami => write!(f, "Futami Ami"),
            IdolList::Mami => write!(f, "Futami Mami"),
            IdolList::Hibiki => write!(f, "Ganaha Hibiki"),
            IdolList::Mirai => write!(f, "Kasuga Mirai"),
            IdolList::Shizuka => write!(f, "Mogami Shizuka"),
            IdolList::Tsubasa => write!(f, "Ibuki Tsubasa"),
            IdolList::Kotoha => write!(f, "Tanaka Kotoha"),
            IdolList::Elena => write!(f, "Shimabara Elena"),
            IdolList::Minako => write!(f, "Satake Minako"),
            IdolList::Megumi => write!(f, "Tokoro Megumi"),
            IdolList::Matsuri => write!(f, "Tokugawa Matsuri"),
            IdolList::Serika => write!(f, "Hakozaki Serika"),
            IdolList::Akane => write!(f, "Nonohara Akane"),
            IdolList::Anna => write!(f, "Mochizuki Anna"),
            IdolList::Roco => write!(f, "Handa Roco"),
            IdolList::Yuriko => write!(f, "Nanao Yuriko"),
            IdolList::Sayoko => write!(f, "Takayama Sayoko"),
            IdolList::Arisa => write!(f, "Matsuda Arisa"),
            IdolList::Umi => write!(f, "Kousaka Umi"),
            IdolList::Iku => write!(f, "Nakatani Iku"),
            IdolList::Tomoka => write!(f, "Tenkubashi Tomoka"),
            IdolList::Emily => write!(f, "Stewart Emily"),
            IdolList::Shiho => write!(f, "Kitazawa Shiho"),
            IdolList::Ayumu => write!(f, "Maihama Ayumu"),
            IdolList::Hinata => write!(f, "Kinoshita Hinata"),
            IdolList::Kana => write!(f, "Yabuki Kana"),
            IdolList::Nao => write!(f, "Yokoyama Nao"),
            IdolList::Chizuru => write!(f, "Nikaido Chizuru"),
            IdolList::Konomi => write!(f, "Baba Konomi"),
            IdolList::Tamaki => write!(f, "Ogami Tamaki"),
            IdolList::Fuka => write!(f, "Toyokawa Fuka"),
            IdolList::Miya => write!(f, "Miyao Miya"),
            IdolList::Noriko => write!(f, "Fukuda Noriko"),
            IdolList::Mizuki => write!(f, "Mayabe Mizuki"),
            IdolList::Karen => write!(f, "Shinomiya Karen"),
            IdolList::Rio => write!(f, "Momose Rio"),
            IdolList::Subaru => write!(f, "Nagayoshi Subaru"),
            IdolList::Reika => write!(f, "Kitakami Reika"),
            IdolList::Momoko => write!(f, "Suou Momoko"),
            IdolList::Julia => write!(f, "Julia"),
            IdolList::Tsumugi => write!(f, "Shiraishi Tsumugi"),
            IdolList::Kaori => write!(f, "Sakuramori Kaorio"),
            IdolList::Shika => write!(f, "Shika"),
            IdolList::Leon => write!(f, "Leon"),
            IdolList::Frederica => write!(f, "Miyamoto Frederica"),
            IdolList::Shiki => write!(f, "Ichinose Shiki")
        }
    }
}

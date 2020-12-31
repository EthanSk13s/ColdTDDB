use std::collections::HashMap;
use regex::Regex;

use super::db;

// TODO: FIX REGEX AND SKILL AT 0

#[derive(Hash, Eq, PartialEq)]
struct Idol<'a> {
    id: i32,
    en_name: &'a str
}

impl<'a> Idol<'a> {
    fn new(id: i32, en_name: &'a str) -> Idol<'a> {
        Idol { id: id, en_name: en_name }
    }
}

type Idols<'a> = HashMap<&'a str, Idol<'a>>;

fn idol_hash() -> Idols<'static> {
    let mut idols: Idols = HashMap::new();

    idols.insert("天海春香", Idol::new(1, "Amami Haruka"));
    idols.insert("如月千早", Idol::new(2, "Kisaragi Chihaya"));
    idols.insert("星井美希", Idol::new(3, "Hoshii Miki"));
    idols.insert("萩原雪歩", Idol::new(4, "Sugawara Yukiho"));
    idols.insert("高槻やよい", Idol::new(5, "Takatsuki Yayoi"));
    idols.insert("菊地真", Idol::new(6, "Kikuchi Makoto"));
    idols.insert("水瀬伊織", Idol::new(7, "Minase Iori"));
    idols.insert("四条貴音", Idol::new(8, "Shijou Takane"));
    idols.insert("秋月律子", Idol::new(9, "Akizuki Ritsuko"));
    idols.insert("三浦あずさ", Idol::new(10, "Miura Azusa"));
    idols.insert("双海亜美", Idol::new(11, "Futami Ami"));
    idols.insert("双海真美", Idol::new(12, "Futami Mami"));
    idols.insert("我那覇響", Idol::new(13, "Ganaha Hibiki"));
    idols.insert("春日未来", Idol::new(14, "Kasuga Mirai"));
    idols.insert("最上静香", Idol::new(15, "Mogami Shizuka"));
    idols.insert("伊吹翼", Idol::new(16, "Ibuki Tsubasa"));
    idols.insert("田中琴葉", Idol::new(17, "Tanaka Kotoha"));
    idols.insert("島原エレナ", Idol::new(18, "Shimabara Elena"));
    idols.insert("佐竹美奈子", Idol::new(19, "Satake Minako"));
    idols.insert("所恵美", Idol::new(20, "Tokoro Megumi"));
    idols.insert("徳川まつり", Idol::new(21, "Tokugawa Matsuri"));
    idols.insert("箱崎星梨花", Idol::new(22, "Hakozaki Serika"));
    idols.insert("野々原茜", Idol::new(23, "Nonohara Akane"));
    idols.insert("望月杏奈", Idol::new(24, "Mochizuki Anna"));
    idols.insert("ロコ", Idol::new(25, "Handa Roco"));
    idols.insert("七尾百合子", Idol::new(26, "Nanao Yuriko"));
    idols.insert("高山紗代子", Idol::new(27, "Takayama Sayoko"));
    idols.insert("松田亜利沙", Idol::new(28, "Matsuda Arisa"));
    idols.insert("高坂海美", Idol::new(29, "Kousaka Umi"));
    idols.insert("中谷育", Idol::new(30, "Nakatani Iku"));
    idols.insert("天空橋朋花", Idol::new(31, "Tenkubashi Tomoka"));
    idols.insert("エミリー", Idol::new(32, "Stewart Emily"));
    idols.insert("北沢志保", Idol::new(33, "Kitazawa Shiho"));
    idols.insert("舞浜歩", Idol::new(34, "Maihama Ayumu"));
    idols.insert("木下ひなた", Idol::new(35, "Kinoshita Hinata"));
    idols.insert("矢吹可奈", Idol::new(36, "Yabuki Kana"));
    idols.insert("横山奈緒", Idol::new(37, "Yokoyama Nao"));
    idols.insert("二階堂千鶴", Idol::new(38, "Nikaido Chizuru"));
    idols.insert("馬場このみ", Idol::new(39, "Baba Konomi"));
    idols.insert("大神環", Idol::new(40, "Ogami Tamaki"));
    idols.insert("豊川風花", Idol::new(41, "Toyokawa Fuka"));
    idols.insert("宮尾美也", Idol::new(42, "Miyao Miya"));
    idols.insert("福田のり子", Idol::new(43, "Fukuda Noriko"));
    idols.insert("真壁瑞希", Idol::new(44, "Makabe Mizuki"));
    idols.insert("篠宮可憐", Idol::new(45, "Shinomiya Karen"));
    idols.insert("百瀬莉緒", Idol::new(46, "Momose Rio"));
    idols.insert("永吉昴", Idol::new(47, "Nagayoshi Subaru"));
    idols.insert("北上麗花", Idol::new(48, "Kitakami Reika"));
    idols.insert("周防桃子", Idol::new(49, "Suou Momoko"));
    idols.insert("ジュリア", Idol::new(50, "Julia"));
    idols.insert("白石紬", Idol::new(51, "Shiraishi Tsumugi"));
    idols.insert("桜守歌織", Idol::new(52, "Sakuramori Kaori"));
    idols.insert("詩花", Idol::new(201, "Shika"));
    idols.insert("玲音", Idol::new(202, "Leon"));
    idols.insert("宮本フレデリカ", Idol::new(204, "Miyamoto Frederica"));
    idols.insert("一ノ瀬志希", Idol::new(205, "Ichinose Shiki"));

    return idols;
}

fn match_name(query: &str) -> &str {
    let idols = idol_hash();

    match idols.get(query) {
       Some(idol) => idol.en_name, 
       _ => "????"
    }
}

pub fn set_name(query: String) -> String {
    lazy_static! {
        static ref SPLIT: Regex = Regex::new(" |　").unwrap();
        static ref RE: Regex = Regex::new("　").unwrap();
    };

    if query.contains("　") || query.contains(" ") {
        let slice: Vec<&str> = SPLIT.split(&query).collect();

        if slice.len() > 2 {
            let names: Vec<&str> = RE.split(&query).collect();
            let tl_name = match_name(names[1]);

            return format!("{} {}", names[0], tl_name);
        } else {
            let names: Vec<&str> = SPLIT.split(&query).collect();
            let tl_name = match_name(names[1]);

            return format!("{} {}", names[0], tl_name);
        };
        
    } else {
        let tl_name = match_name(&query);
        return String::from(tl_name);
    };
}

pub fn tl_skill(skill: &db::JsonSkill) -> String {
    let interval = format_interval(skill.interval, skill.probability);
    let duration = format_duration(skill.duration);

    let eval1 = match_evaluations(skill.evaluation);
    let eval2 = match_evaluations(skill.evaluation2);

    let effect = match_effect(
        skill.effect_id, String::from(eval1),
        String::from(eval2), skill.value.clone()
    );

    if skill.effect_id == 4 {
        return format!("{} {} {}", interval, effect, duration);
    } else if skill.effect_id == 0 {
        return String::from("null")
    };

    format!("{} {} {}", interval, effect, duration)
}

pub fn tl_center_skill(skill: &db::JsonCenter) -> String {
    if skill.description != String::from("スキルなし") {
        let idol_type = match_idol_type(skill.idol_type);
        let attribute = match_attributes(skill.attribute);

        let first_cond = format_center(idol_type, attribute, skill.value);

        if skill.song_type != 0 {
            let song = match_song_type(skill.song_type);
            let second_cond = format_song(song, skill.value_2);

            return format!("{} {}", first_cond, second_cond);
        } else {
            return first_cond;
        };
    } else {
        String::from("null")
    }
    
}

fn match_idol_type(id: i16) -> &'static str {
    match id {
        1 => "Princess",
        2 => "Fairy",
        3 => "Angel",
        4 => "All-Type",
        _ => "Unknown"
    }
}

fn match_song_type(id: i16) -> &'static str {
    match id {
        1 => "Princess",
        2 => "Fairy",
        3 => "Angel",
        4 => "All",
        _ => "Unknown"
    }
}

fn match_attributes(id: i16) -> &'static str {
    match id {
        1 => "vocal",
        2 => "dance",
        3 => "visual",
        4 => "all appeal",
        5 => "life",
        6 => "skill rate up",
        _ => "Unknown"
    }
}

fn match_evaluations(id: i16) -> &'static str {
    match id {
        1 => "Perfect",
        2 => "Perfect/Great",
        3 => "Great",
        4 => "Great/Fast/Good/Slow",
        5 => "Perfect/Great/Good",
        6 => "Perfect/Gread/Good/Fast/Slow",
        7 => "Great/Good",
        _ => "Unknown"
    }
}

fn match_skill_type(id: i16) -> String {
    match id {
        1 => String::from("Score Up"),
        2 => String::from("Combo Bonus"),
        3 => String::from("Life Recovery"),
        4 => String::from("Damage Guard"),
        5 => String::from("Maintain Combo"),
        6 => String::from("Judgment Strengthening"),
        7 => String::from("Double Boost"),
        8 => String::from("Multi-Up"),
        10 => String::from("Overclock"),
        11 => String::from("Overrun"), // Got to double check this, my katakana reading sucks
        _ => String::from("Unknown Skill")
    }
}

fn match_effect(id: i16, eval1: String,
     eval2: String, value: Vec<i32>) -> String {
    match id {
        1 => format!("of increasing {} score by {}%", eval1, value[0]),
        2 => format!("of increasing the combo bonus by {}", value[0]),
        3 => format!("of recovering {} lives while hitting {}", value[0], eval1),
        4 => String::from("of not losing life"),
        5 => format!("of maintaining the combo, while hitting {}", eval1),
        6 => format!("of converting {} into Perfect", eval1),
        7 => format!("of increasing the {} score by {} and the combo bonus by {}%", eval1, value[0], value[1]),
        8 => format!("of increasing {} score by {}% and recovering {} life with every {}", eval1, value[0], value[1], eval2),
        10 => format!("of consuming {} lives, and increasing {} score by {}%", value[1], eval1, value[0]),
        11 => format!("of consuming {} lives, and increasing combo bonus by {}%", value[1], value[0]),
        _ => String::from("No TL found.")
    }
}

fn format_interval(interval: i16,
     probability: i16) -> String {
    format!(
        "Every {} seconds, there is a {}% chance",
         interval.to_string(),
         probability.to_string())
}

fn format_duration(duration: i16) -> String {
    format!("for {} seconds", duration.to_string())
}

fn format_center(idol_type: &str, attr: &str, value: i32) -> String {
    format!(
        "{} idols' {} value is increased by {}%",
        idol_type, attr, value.to_string()
    )
}

fn format_song(attr: &str, value: i32) -> String {
    format!(
        "If playing an {} song, an additional {}% is added.",
        attr, value.to_string()
    )
}
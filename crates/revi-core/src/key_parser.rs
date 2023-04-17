use revi_ui::Key;
use std::{iter::Peekable, str::Chars};
// use revi_ui::Keys;
// struct KeyParser {
//     text: Vec<char>,
//     keys: Vec<Keys>,
//     numbers: Vec<u16>,
//     ip: usize,
// }
//
// fn strip_it(key: &str) -> Option<&str> {
//     key.strip_prefix('<')
//         .and_then(|n| n.strip_suffix('>'))
// }
//
// fn decode_mod_keys<'a>(key: &str) -> Keys {
//     strip_it(key)
//         .and_then(|n| n.split_once('-'))
//         .and_then(|(m, k)| {
//             let key = Key::from(k);
//             let modk = Key::from(lookup_mod_key(m));
//             Some(Keys::KeyAndMod { key, modk })
//
//         })
//         .or(strip_it(key).map(|k| Keys::Key(Key::from(k))))
//         .unwrap_or( Keys::Key(Key::from(key)))
// }
//
// fn lookup_mod_key(modk: &str) -> &str {
//     match modk {
//         "c" => "ctrl",
//         "a" => "alt",
//         _ => modk,
//     }
// }

fn specal_keys<'a>(stream: &mut Peekable<Chars<'a>>) -> Vec<Key> {
    let mut string = String::new();
    while let Some(c) = stream.next_if(|c| c != &'>') {
        string.push(c);
    }
    let _ = stream.next();
    if string.contains("-") {
        let lr = string.split("-").collect::<Vec<&str>>();
        let modifier = lr[0].to_lowercase();
        let modifier = match modifier.as_str() {
            "c" => "ctrl",
            "a" => "alt",
            m => m,
        };
        return vec![
            Key::from(modifier),
            Key::from(lr[1].chars().collect::<Vec<char>>()[0]),
        ];
    }
    vec![Key::from(string.as_str())]
}

pub fn string_to_key(keys_string: &str) -> Vec<Key> {
    let mut stream = keys_string.chars().peekable();
    let mut keys = Vec::new();
    while let Some(c) = stream.next() {
        match c {
            '<' => keys.append(&mut specal_keys(&mut stream)),
            _ => keys.push(Key::from(c)),
        }
    }
    keys
}

#[cfg(test)]
mod test {
    use super::*;
    use revi_ui::Keys;
    // #[test]
    // fn test_decode_mod_keys() {
    //     let keys = decode_mod_keys("<c-a>");
    //     assert_eq!(keys, Keys::KeyAndMod{key:Key::LA, modk:Key::Ctrl});
    // }

    #[test]
    fn specal_keys_parser() {
        let string = "esc>";
        assert_eq!(vec![Key::Esc], specal_keys(&mut string.chars().peekable()));

        // let string = "<esc>";
        // assert_eq!(Keys::Key(Key::Esc), decode_mod_keys(string));

        let string = "space>";
        assert_eq!(
            vec![Key::Space],
            specal_keys(&mut string.chars().peekable())
        );

        let string = "C-c>";
        assert_eq!(
            vec![Key::Ctrl, Key::LC],
            specal_keys(&mut string.chars().peekable())
        );
    }
    #[test]
    fn test_parse_string_space_a() {
        let string = "<space>a";
        assert_eq!(vec![Key::Space, Key::LA], string_to_key(string));
    }

    #[test]
    fn parse_ctrl_a() {
        let string = "<C-a>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Ctrl, Key::LA], string_to_key(string));
    }

    #[test]
    fn parse_tab() {
        let string = "<tab>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Tab], string_to_key(string));
    }

    #[test]
    fn parse_esc() {
        let string = "<esc>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Esc], string_to_key(string));
    }

    #[test]
    fn parse_ctrl_h() {
        let string = "<C-h>";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::Ctrl, Key::LH], string_to_key(string));
    }

    #[test]
    fn parse_i() {
        let string = "i";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::LI], string_to_key(string));
    }

    #[test]
    #[allow(non_snake_case)]
    fn parse_I() {
        let string = "I";
        eprintln!("{:?}", string);
        assert_eq!(vec![Key::UI], string_to_key(string));
    }
}

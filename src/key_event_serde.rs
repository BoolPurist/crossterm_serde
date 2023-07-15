use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use once_cell::sync::Lazy;
use serde::Serialize;
use serde::{de, ser, Deserialize, Deserializer, Serializer};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(remote = "KeyEvent")]
pub struct SerDeConfigKeyEvent {
    #[serde(with = "serde_key_code")]
    code: KeyCode,
    #[serde(default = "default_modifiers")]
    #[serde(with = "serde_key_modifier")]
    modifiers: KeyModifiers,
    #[serde(skip)]
    #[serde(default = "default_event_kind")]
    kind: KeyEventKind,
    #[serde(skip)]
    #[serde(default = "default_event_state")]
    state: KeyEventState,
}

fn default_modifiers() -> KeyModifiers {
    KeyModifiers::NONE
}
fn default_event_kind() -> KeyEventKind {
    KeyEventKind::Press
}
fn default_event_state() -> KeyEventState {
    KeyEventState::NONE
}

mod serde_key_code {
    use std::borrow::Cow;

    use super::*;
    use crossterm::event::KeyCode;

    static KEYWORDS: Lazy<HashMap<&str, KeyCode>> = Lazy::new(|| {
        HashMap::from([
            ("Backspace", KeyCode::Backspace),
            ("Enter", KeyCode::Enter),
            ("Left", KeyCode::Left),
            ("Right", KeyCode::Right),
            ("Up", KeyCode::Up),
            ("Down", KeyCode::Down),
            ("Home", KeyCode::Home),
            ("End", KeyCode::End),
            ("PageUp", KeyCode::PageUp),
            ("PageDown", KeyCode::PageDown),
            ("Tab", KeyCode::Tab),
            ("BackTab", KeyCode::BackTab),
            ("Delete", KeyCode::Delete),
            ("Insert", KeyCode::Insert),
            ("Null", KeyCode::Null),
            ("Esc", KeyCode::Esc),
            ("CapsLock", KeyCode::CapsLock),
            ("ScrollLock", KeyCode::ScrollLock),
            ("NumLock", KeyCode::NumLock),
            ("PrintScreen", KeyCode::PrintScreen),
            ("Pause", KeyCode::Pause),
            ("Menu", KeyCode::Menu),
            ("KeypadBegin", KeyCode::KeypadBegin),
        ])
    });

    static KEYWORDS_REV: Lazy<HashMap<KeyCode, &str>> = Lazy::new(|| {
        let swaped = KEYWORDS
            .iter()
            .map(|(&to_right, &to_left)| (to_left, to_right));
        HashMap::from_iter(swaped)
    });

    pub fn serialize<S>(code: &KeyCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let content = key_code_to_text(code)?;
        serializer.serialize_str(&content)
    }

    fn key_code_to_text<E>(code: &KeyCode) -> Result<Cow<'static, str>, E>
    where
        E: ser::Error,
    {
        match code {
            KeyCode::Char(char) => Ok(Cow::Owned(char.to_string())),
            code => {
                if let Some(value) = KEYWORDS_REV.get(code) {
                    Ok(Cow::Borrowed(value))
                } else {
                    Err(ser::Error::custom(
                        "One char must be provided or a valie keyword for a key like (Up)",
                    ))
                }
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.trim().to_string();
        parse_key_code(&s)
    }

    fn parse_key_code<E>(text: &str) -> Result<KeyCode, E>
    where
        E: de::Error,
    {
        const ERROR_MESSAGE: &str = "One char or a certain keyword must be provided";

        if text.is_empty() {
            Err(de::Error::custom(ERROR_MESSAGE))
        } else if text.len() == 1 {
            let key_code = KeyCode::Char(text.chars().next().unwrap());
            Ok(key_code)
        } else if let Some(valid_keyword) = KEYWORDS.get(text) {
            Ok(*valid_keyword)
        } else {
            Err(de::Error::custom(ERROR_MESSAGE))
        }
    }

    #[cfg(test)]
    mod testing {
        use super::*;

        #[test]
        fn should_produce_text_as_key_code() {
            assert_case(KeyCode::Char('a'), "a");
            assert_case(KeyCode::Char('A'), "A");
            assert_case(KeyCode::Char('/'), "/");
            assert_case(KeyCode::Up, "Up");
            assert_case(KeyCode::Enter, "Enter");
            fn assert_case(input: KeyCode, expected: &str) {
                let actual = key_code_to_text::<ron::Error>(&input).unwrap();
                assert_eq!(expected, &actual);
            }
        }
        #[test]
        fn should_parse_valid_key_code() {
            assert_case("a", KeyCode::Char('a'));
            assert_case("A", KeyCode::Char('A'));
            assert_case("/", KeyCode::Char('/'));
            assert_case("Up", KeyCode::Up);
            assert_case("Enter", KeyCode::Enter);
            fn assert_case(input: &str, expected: KeyCode) {
                let actual = parse_key_code::<ron::Error>(input).unwrap();
                assert_eq!(expected, actual);
            }
        }
    }
}
pub mod serde_key_modifier {
    use crossterm::event::KeyModifiers;

    use super::*;
    use serde::{de, Deserialize, Deserializer, Serializer};

    const SEPERATOR: &str = "+";
    const NONE: &str = "NONE";

    const SHIFT: &str = "SHIFT";
    const CONTROL: &str = "CONTROL";
    const SUPER: &str = "SUPER";
    const ALT: &str = "ALT";
    const HYPER: &str = "HYPER";
    const META: &str = "META";

    static KEYWORD: Lazy<HashMap<&str, KeyModifiers>> = Lazy::new(|| {
        HashMap::from([
            (SHIFT, KeyModifiers::SHIFT),
            (CONTROL, KeyModifiers::CONTROL),
            (ALT, KeyModifiers::ALT),
            (SUPER, KeyModifiers::SUPER),
            (HYPER, KeyModifiers::HYPER),
            (META, KeyModifiers::META),
            (NONE, KeyModifiers::NONE),
        ])
    });

    macro_rules! push_if_contains {
        ($m:ident, $v:ident, $e:ident) => {
            if $m.contains(KeyModifiers::$e) {
                $v.push(stringify!($e));
            }
        };
    }

    fn bits_to_strs(modif: &KeyModifiers) -> Vec<&str> {
        let mut to_return = Vec::new();
        push_if_contains!(modif, to_return, ALT);
        push_if_contains!(modif, to_return, CONTROL);
        push_if_contains!(modif, to_return, SHIFT);
        push_if_contains!(modif, to_return, SUPER);
        push_if_contains!(modif, to_return, HYPER);
        if modif.is_empty() {
            to_return.push(NONE);
        }
        to_return
    }

    pub fn serialize<S>(modifier: &KeyModifiers, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let seq = bits_to_strs(modifier);
        serializer.serialize_str(&seq.join(SEPERATOR))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<KeyModifiers, D::Error>
    where
        D: Deserializer<'de>,
    {
        let text = String::deserialize(deserializer)?;
        parse_key_modifier(&text)
    }

    fn parse_key_modifier<E>(text: &str) -> Result<KeyModifiers, E>
    where
        E: de::Error,
    {
        let text = text.trim();

        if text.is_empty() {
            return Err(de::Error::custom(
                "Need to provide at least keyword for the key modifier",
            ));
        }

        let mut result = KeyModifiers::NONE;
        for next in text.split(SEPERATOR) {
            let keyword = KEYWORD
                .get(next)
                .ok_or_else(|| de::Error::custom(format!("{} is not a valid keyword", next)))?;
            result |= *keyword;
        }

        Ok(result)
    }

    #[cfg(test)]
    mod testing {

        use super::*;
        #[test]
        fn should_accept_valid_key_modifiers() {
            assert_case(
                format!("{}+{}", ALT, CONTROL),
                KeyModifiers::CONTROL | KeyModifiers::ALT,
            );
            assert_case(
                format!("{}+{}+{}", META, NONE, SUPER),
                KeyModifiers::META | KeyModifiers::SUPER,
            );
            assert_case(format!("{}", NONE), KeyModifiers::NONE);
            fn assert_case(input: String, expected: KeyModifiers) {
                let actual: Result<KeyModifiers, ron::Error> = parse_key_modifier(&input);
                assert_eq!(expected, actual.unwrap());
            }
        }
        #[test]
        fn should_deny_invalid_key_modifiers() {
            assert_case(format!(""));
            assert_case(format!("AL"));
            assert_case(format!("ALT+Z"));
            fn assert_case(input: String) {
                let actual: Result<KeyModifiers, ron::Error> = parse_key_modifier(&input);
                assert!(actual.is_err());
            }
        }
        #[test]
        fn should_convert_bits_strs() {
            let expected = &[ALT, CONTROL];
            let input = KeyModifiers::ALT | KeyModifiers::CONTROL;
            let actual = bits_to_strs(&input);
            assert_eq!(expected.as_slice(), actual.as_slice());
        }
        #[test]
        fn should_convert_none_to_one_none() {
            let expected = &[NONE];
            let input = KeyModifiers::empty();
            let actual = bits_to_strs(&input);
            assert_eq!(expected.as_slice(), actual.as_slice());
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[derive(Debug, Serialize, Deserialize)]
    pub struct KeyBoard {
        #[serde(with = "SerDeConfigKeyEvent")]
        move_up: KeyEvent,
        #[serde(with = "SerDeConfigKeyEvent")]
        move_down: KeyEvent,
        #[serde(with = "SerDeConfigKeyEvent")]
        move_left: KeyEvent,
        #[serde(with = "SerDeConfigKeyEvent")]
        move_right: KeyEvent,
    }

    const RON_INPUT: &str = r#"
(
    move_up: (
        code: "Up",
        modifiers: "NONE",
    ),
    move_down: (
        code: "Down",
        modifiers: "ALT",
    ),
    move_left: (
        code: "Left",
        modifiers: "ALT+CONTROL",
    ),
    move_right: (
        code: "Right",
        modifiers: "SUPER",
    ),
)
    "#;
    fn input() -> KeyBoard {
        KeyBoard {
            move_up: KeyEvent::new(KeyCode::Up, KeyModifiers::NONE),
            move_down: KeyEvent::new(KeyCode::Down, KeyModifiers::ALT),
            move_left: KeyEvent::new(KeyCode::Left, KeyModifiers::ALT | KeyModifiers::CONTROL),
            move_right: KeyEvent::new(KeyCode::Right, KeyModifiers::NONE | KeyModifiers::SUPER),
        }
    }
    #[test]
    fn test_serlize() {
        let input = input();

        let actual = ron::ser::to_string_pretty(&input, ron::ser::PrettyConfig::default()).unwrap();
        insta::assert_display_snapshot!(actual);
    }
    #[test]
    fn test_derserlize() {
        let actual: KeyBoard = ron::from_str(RON_INPUT).unwrap();
        insta::assert_ron_snapshot!(actual);
    }
}

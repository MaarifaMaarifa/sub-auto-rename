use std::ffi::OsStr;

/// Whether or not Episode signature matches
#[derive(Debug, PartialEq)]
pub enum MatchSignature {
    Match,
    NoMatch,
}

/// Checks if the two file names have the same episodic signature, that is S01E02 signature
/// matches on both files, return the match signature
pub fn episode_name_signature_check(first_name: &OsStr, second_name: &OsStr) -> MatchSignature {
    let first_name = first_name.to_string_lossy().to_string().to_lowercase();
    let second_name = second_name.to_string_lossy().to_string().to_lowercase();

    let first_name_season = get_signature_value(SignatureType::Season, &first_name);
    let first_name_episode = get_signature_value(SignatureType::Episode, &first_name);
    let second_name_season = get_signature_value(SignatureType::Season, &second_name);
    let second_name_episode = get_signature_value(SignatureType::Episode, &second_name);

    let mut seasons_matched = false;
    let mut episodes_matched = false;

    if let Some(first_name_season) = first_name_season {
        if let Some(second_name_season) = second_name_season {
            if first_name_season == second_name_season {
                seasons_matched = true
            }
        }
    }
    if let Some(first_name_episode) = first_name_episode {
        if let Some(second_name_episode) = second_name_episode {
            if first_name_episode == second_name_episode {
                episodes_matched = true
            }
        }
    }

    if seasons_matched && episodes_matched {
        MatchSignature::Match
    } else {
        MatchSignature::NoMatch
    }
}

enum SignatureType {
    Season,
    Episode,
}

/// Returns the value of season/episode in the given string, this is specified
/// via it's signature type parameter
fn get_signature_value(signature_type: SignatureType, name: &str) -> Option<u32> {
    let char_to_check = match signature_type {
        SignatureType::Season => 's',
        SignatureType::Episode => 'e',
    };

    let mut value = None;

    for chunk in name.split(char_to_check) {
        let value_str: String = chunk.chars().take_while(|x| x.is_numeric()).collect();

        if !value_str.is_empty() {
            // SAFETY: all the characters in the string have been checked if they are numeric
            // hence calling unwrap here is safe
            value = Some(value_str.parse::<u32>().unwrap());
            break
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode_name_signature_check_test() {
        let name_1 = OsStr::new("Hellos01e02mov");
        let name_2 = OsStr::new("Hellos01e02WebSub");
        let name_3 = OsStr::new("Hellos01 e02mov");
        let name_4 = OsStr::new("HelloWorld");

        let match_signature_1 = episode_name_signature_check(name_1, name_2);
        let match_signature_2 = episode_name_signature_check(name_1, name_3);
        let match_signature_3 = episode_name_signature_check(name_1, name_4);

        assert_eq!(match_signature_1, MatchSignature::Match);
        assert_eq!(match_signature_2, MatchSignature::Match);
        assert_eq!(match_signature_3, MatchSignature::NoMatch);
    }

    #[test]
    fn episode_name_signature_check_realmatch_test() {
        let name_1 = OsStr::new("some.video.file.S04 E01.mp4");
        let name_2 = OsStr::new("some.video.file.S04E01.srt");

        let name_3 = OsStr::new("some.video.file.S04 E10.mp4");
        let name_4 = OsStr::new("some.video.file.S04E10.srt");

        let match_signature_1 = episode_name_signature_check(name_1, name_2);
        let match_signature_2 = episode_name_signature_check(name_3, name_4);

        assert_eq!(match_signature_1, MatchSignature::Match);
        assert_eq!(match_signature_2, MatchSignature::Match);
    }

    #[test]
    fn episode_name_signature_check_realnomatch_failure_test() {
        let name_1 = OsStr::new("some.video.file.S04 E01.mp4");
        let name_2 = OsStr::new("some.video.file.S04E01.srt");

        let name_3 = OsStr::new("some.video.file.S04 E10.mp4");
        let name_4 = OsStr::new("some.video.file.S04E10.srt");

        let match_signature_1 = episode_name_signature_check(name_1, name_3);
        let match_signature_2 = episode_name_signature_check(name_2, name_4);

        assert_eq!(match_signature_1, MatchSignature::NoMatch);
        assert_eq!(match_signature_2, MatchSignature::NoMatch);
    }

    #[test]
    fn get_signature_val_for_episode_test() {
        let file_str = "hellos01e23.mov";
        assert_eq!(get_signature_value(SignatureType::Episode, file_str).unwrap(), 23);
    }
    #[test]
    fn get_signature_val_for_season_test() {
        let file_str = "hellos01e23.mov";
        assert_eq!(get_signature_value(SignatureType::Season, file_str).unwrap(), 1);
    }
}

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

    let first_name_sig_ranges = get_season_episode_sig_range(&first_name);
    let second_name_sig_ranges = get_season_episode_sig_range(&second_name);

    if first_name_sig_ranges.is_none() || second_name_sig_ranges.is_none() {
        return MatchSignature::NoMatch;
    }

    let (first_name_season_range, first_name_episode_range) = first_name_sig_ranges.unwrap();
    let (second_name_season_range, second_name_episode_range) = second_name_sig_ranges.unwrap();

    let first_name_season_string = first_name_season_range.get_section_from_str(&first_name);
    let first_name_episode_string = first_name_episode_range.get_section_from_str(&first_name);
    let second_name_season_string = second_name_season_range.get_section_from_str(&second_name);
    let second_name_episode_string = second_name_episode_range.get_section_from_str(&second_name);

    if first_name_episode_string == second_name_episode_string
        && first_name_season_string == second_name_season_string
    {
        MatchSignature::Match
    } else {
        MatchSignature::NoMatch
    }
}

/// Returns the Season Signature range and Episode signature range as a Optional tuple on the provided file name string
/// of the signature someepisodeS02E01
fn get_season_episode_sig_range(name: &str) -> Option<(SignatureRange, SignatureRange)> {
    if let Some(season_sig_range) = signature_range(SignatureType::Season, name) {
        signature_range(SignatureType::Episode, name)
            .map(|episode_sig_range| (season_sig_range, episode_sig_range))
    } else {
        None
    }
}

/// Struct representing the range of season or episode signature
/// Let's say you are given name someepisodeS02E01, it's season range will cover S02
/// and it's episode range will cover E01
///
/// # Point to note
/// This range is inclusive
#[derive(Debug, PartialEq)]
struct SignatureRange(usize, usize);

impl SignatureRange {
    /// Create a new instance of signatureRange
    ///
    /// # Panics
    /// This method panics when start is greater than end
    fn new(start: usize, end: usize) -> Self {
        if start > end {
            panic!("start is greater than end. start: {}, end: {}", start, end)
        }
        Self(start, end)
    }

    /// Get a section of a str that has the range a SignatureRange self as a String
    fn get_section_from_str(&self, string: &str) -> String {
        let range_diff = self.1 - self.0;
        string
            .chars()
            .skip(self.0)
            .take(range_diff)
            .collect::<String>()
    }
}

enum SignatureType {
    Season,
    Episode,
}

/// Returns a Signature range on the provided name based on Signature type provided
/// i.e of season or episode
fn signature_range(signature_type: SignatureType, name: &str) -> Option<SignatureRange> {
    let char_to_check = match signature_type {
        SignatureType::Season => 's',
        SignatureType::Episode => 'e',
    };

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;

    name.split(char_to_check)
        .take_while(|chunk| {
            let last_numeric_index = chunk.chars().take_while(|x| x.is_numeric()).count();

            if last_numeric_index != 0 {
                end = Some(last_numeric_index)
            }
            end.is_none()
        })
        .for_each(|chunk| {
            if let Some(ref mut val) = start {
                *val += chunk.len() + 1
            } else {
                start = Some(chunk.len())
            }
        });

    if let Some(start) = start {
        if let Some(end) = end {
            return Some(SignatureRange::new(start, end + start))
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod signature_range_fn_tests {
        use super::*;

        #[test]
        fn signature_range_fn_test() {
            let expected_season_range = SignatureRange(5, 7);
            let expected_episode_range = SignatureRange(8, 10);
            let name = "Hellos01e02.mp4";

            assert_eq!(
                signature_range(SignatureType::Season, &name),
                Some(expected_season_range)
            );
            assert_eq!(
                signature_range(SignatureType::Episode, &name),
                Some(expected_episode_range)
            );
        }

        #[test]
        fn signature_range_fn_with_space_test() {
            let expected_season_range = SignatureRange(5, 7);
            let expected_episode_range = SignatureRange(9, 11);
            let name = "Hellos01 e02.mp4";

            assert_eq!(
                signature_range(SignatureType::Season, &name),
                Some(expected_season_range)
            );
            assert_eq!(
                signature_range(SignatureType::Episode, &name),
                Some(expected_episode_range)
            );
        }

        #[test]
        fn signature_range_without_extension_fn_test() {
            let expected_season_range = SignatureRange(5, 7);
            let expected_episode_range = SignatureRange(8, 10);
            let name = "Hellos01e02";

            assert_eq!(
                signature_range(SignatureType::Season, &name).unwrap(),
                expected_season_range
            );
            assert_eq!(
                signature_range(SignatureType::Episode, &name).unwrap(),
                expected_episode_range
            );
        }

        #[test]
        #[should_panic]
        fn signature_range_fn_failure_test() {
            let name = "Hellos01.mp4";
            signature_range(SignatureType::Episode, &name).unwrap();
        }

        #[test]
        fn signature_range_fn_with_many_s_test() {
            let expected_season_range = SignatureRange(5, 7);
            let expected_episode_range = SignatureRange(9, 11);
            let name = "hellss01 e02.mp4";

            assert_eq!(
                signature_range(SignatureType::Season, &name),
                Some(expected_season_range)
            );
            assert_eq!(
                signature_range(SignatureType::Episode, &name),
                Some(expected_episode_range)
            );
        }

        #[test]
        fn signature_range_fn_with_many_e_test() {
            let expected_season_range = SignatureRange(5, 7);
            let expected_episode_range = SignatureRange(9, 11);
            let name = "helees01 e02.mp4";

            assert_eq!(
                signature_range(SignatureType::Season, &name),
                Some(expected_season_range)
            );
            assert_eq!(
                signature_range(SignatureType::Episode, &name),
                Some(expected_episode_range)
            );
        }
    }

    #[test]
    fn get_section_from_str_test() {
        let season_range = SignatureRange(5, 8);
        let episode_range = SignatureRange(8, 11);
        let expected_season_signature = "s01";
        let expected_episode_signature = "e02";
        let name = "Hellos01e02.mp4";

        let season_signature = season_range.get_section_from_str(&name);
        let episode_signature = episode_range.get_section_from_str(&name);

        assert_eq!(season_signature, expected_season_signature);
        assert_eq!(episode_signature, expected_episode_signature);
    }

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
}

use thiserror::Error;

const FM_DELIMITER: &'static str = "---\n";

/// [`find_yaml_block`] is an util function that will output the starting and ending characters delimiter
/// for a Yaml front matter block if there is one in the given file.
pub(crate) fn find_yaml_block(file_data: &str) -> Option<(usize, usize)> {
    if file_data.starts_with(FM_DELIMITER) {
        let slice_after_marker = &file_data[4..];
        let fm_end = slice_after_marker.find(FM_DELIMITER);
        if !fm_end.is_none() {
            let fm_end = fm_end.unwrap();
            return Some((4, fm_end + 4));
        }
    }

    None
}

#[derive(Error, Debug)]
/// Type for errors
pub(crate) enum ValidatorError {
    /// Thrown when no file path is specified
    #[error("file path is not specified")]
    NoFilePath,
    /// Thrown when a data is not of desired format
    #[error("date is not iso8601 compatible")]
    InvalidDateFormat,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn none_on_no_fm() {
        let missing_fm = "We are missing something";

        let res = find_yaml_block(missing_fm);

        assert!(res.is_none());
    }

    #[test]
    fn none_on_wrong_delimiter() {
        let missing_fm = "----\nhip: 102\ntitle: MyTitle\ndescription: Description is one full (short) sentence\nauthor: Me\nstatus: Draft\ncreated: 2021-06-06\n----\n";

        let res = find_yaml_block(missing_fm);

        assert!(res.is_none());
    }

    #[test]
    fn retrieve_start_end_index() {
        let missing_fm = "---\nhip: 102\ntitle: MyTitle\ndescription: Description is one full (short) sentence\nauthor: Me\nstatus: Draft\ncreated: 2021-06-06\n---\n";

        let res = find_yaml_block(missing_fm);

        assert_eq!((4, 127), res.unwrap());
    }
}

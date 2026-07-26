use super::*;

pub(super) fn environment_selection_error_message(err: SoryErr) -> String {
    match err {
        SoryErr::InvalidRequest(message) => message,
        err => err.to_string(),
    }
}

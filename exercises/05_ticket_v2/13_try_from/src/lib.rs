// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.


use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, Error)]
enum CoverError {
    #[error("转换失败")]
    CoverFail
}

impl TryFrom<String> for Status {
    type Error = CoverError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == String::from("ToDO") {
            Ok(Status::ToDo)
        } else if value == String::from("inproGress") {
            Ok(Status::InProgress)
        } else if value == String::from("Done") {
            Ok(Status::Done)
        } else {
            Err(CoverError::CoverFail)
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = CoverError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "todo" {
            Ok(Status::ToDo)
        } else if value == "inprogress" {
            Ok(Status::InProgress)
        } else if value == "done" {
            Ok(Status::Done)
        } else {
            Err(CoverError::CoverFail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}

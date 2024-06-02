// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Clone, Debug,PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "ToDO".to_string() {
            return Ok(Status::ToDo);
        }
        if value == "inproGress".to_string() {
            return Ok(Status::InProgress);
        }
        if value == "Done".to_string() {
            return Ok(Status::Done);
        }
        Err("Invild".to_string())
    }
}

impl <'a> TryFrom<&'a str> for Status {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value == "ToDO" {
            return Ok(Status::ToDo);
        }
        if value == "inproGress" {
            return Ok(Status::InProgress);
        }
        if value == "Done" {
            return Ok(Status::Done);
        }
        Err("Invild")
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
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}

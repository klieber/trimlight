use crate::error::TrimlightError;

/// Parse time string in HH:MM format
pub fn parse_time(time: &str) -> Result<(i32, i32), TrimlightError> {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 2 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid time format. Use HH:MM".to_string(),
        });
    }

    let hours = parts[0].trim().parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid hours".to_string(),
    })?;

    let minutes = parts[1].trim().parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid minutes".to_string(),
    })?;

    if hours < 0 || hours > 23 || minutes < 0 || minutes > 59 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid time values".to_string(),
        });
    }

    Ok((hours, minutes))
}

/// Parse date string in MM-DD format
pub fn parse_date(date: &str) -> Result<(i32, i32), TrimlightError> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 2 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid date format. Use MM-DD".to_string(),
        });
    }

    let month = parts[0].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid month".to_string(),
    })?;

    let day = parts[1].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid day".to_string(),
    })?;

    if month < 1 || month > 12 || day < 1 || day > 31 {
        return Err(TrimlightError::ApiError {
            code: 400,
            message: "Invalid date values".to_string(),
        });
    }

    Ok((month, day))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_valid() {
        // Test valid time formats
        assert_eq!(parse_time("00:00").unwrap(), (0, 0));
        assert_eq!(parse_time("23:59").unwrap(), (23, 59));
        assert_eq!(parse_time("12:30").unwrap(), (12, 30));
        assert_eq!(parse_time("09:05").unwrap(), (9, 5));
    }

    #[test]
    fn test_parse_time_invalid_format() {
        // Test invalid formats
        assert!(matches!(
            parse_time("12:30:00"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time format. Use HH:MM"
        ));
        assert!(matches!(
            parse_time("12"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time format. Use HH:MM"
        ));
        assert!(matches!(
            parse_time("12:"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid minutes"
        ));
        assert!(matches!(
            parse_time(":30"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid hours"
        ));
    }

    #[test]
    fn test_parse_time_invalid_values() {
        // Test invalid hour values
        assert!(matches!(
            parse_time("24:00"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time values"
        ));
        assert!(matches!(
            parse_time("-1:00"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time values"
        ));

        // Test invalid minute values
        assert!(matches!(
            parse_time("12:60"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time values"
        ));
        assert!(matches!(
            parse_time("12:-1"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid time values"
        ));
    }

    #[test]
    fn test_parse_date_valid() {
        // Test valid date formats
        assert_eq!(parse_date("01-01").unwrap(), (1, 1));
        assert_eq!(parse_date("12-31").unwrap(), (12, 31));
        assert_eq!(parse_date("02-28").unwrap(), (2, 28));
        assert_eq!(parse_date("09-05").unwrap(), (9, 5));
    }

    #[test]
    fn test_parse_date_invalid_format() {
        // Test invalid formats
        assert!(matches!(
            parse_date("2024-01-01"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date format. Use MM-DD"
        ));
        assert!(matches!(
            parse_date("12"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date format. Use MM-DD"
        ));
        assert!(matches!(
            parse_date("12-"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid day"
        ));
        assert!(matches!(
            parse_date("-31"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid month"
        ));
    }

    #[test]
    fn test_parse_date_invalid_values() {
        // Test invalid month values
        assert!(matches!(
            parse_date("00-01"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date values"
        ));
        assert!(matches!(
            parse_date("13-01"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date values"
        ));

        // Test invalid day values
        assert!(matches!(
            parse_date("01-00"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date values"
        ));
        assert!(matches!(
            parse_date("01-32"),
            Err(TrimlightError::ApiError {
                code: 400,
                message
            }) if message == "Invalid date values"
        ));
    }
}

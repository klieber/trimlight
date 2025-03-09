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

    let hours = parts[0].parse::<i32>().map_err(|_| TrimlightError::ApiError {
        code: 400,
        message: "Invalid hours".to_string(),
    })?;

    let minutes = parts[1].parse::<i32>().map_err(|_| TrimlightError::ApiError {
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

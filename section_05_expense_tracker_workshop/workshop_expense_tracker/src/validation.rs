use crate::config::{MAX_AMOUNT, MAX_CATEGORY_LENGTH, MIN_AMOUNT};
use crate::errors::ExpenseError;
use chrono::NaiveDate;

// Validate Amount
pub fn validate_amount(amount: f64) -> Result<f64, ExpenseError> {

    if amount < MIN_AMOUNT {
        return Err(ExpenseError::ParseError(format!(
            "จำนวนเงินต้องมากกว่า {MIN_AMOUNT:.2}"
        )));
    }

    if amount > MAX_AMOUNT {
        return Err(ExpenseError::ParseError(format!(
            "จำนวนเงินต้องน้อยกว่า {MAX_AMOUNT:.2}"
        )));
    }

    Ok(amount)

}

// Valide Category
pub fn validate_category(category: &str) -> Result<String, ExpenseError> {
    let trimmed = category.trim();

    if trimmed.is_empty() {
        return Err(ExpenseError::ParseError(
            "หมวดหมู่ไม่สามารถเป็นค่าว่างได้".to_string(),
        ));
    }

    if trimmed.len() > MAX_CATEGORY_LENGTH {
        return Err(ExpenseError::ParseError(format!(
            "หมวดหมู่ต้องไม่เกิน {MAX_CATEGORY_LENGTH} ตัวอักษร"
        )));
    }

    Ok(trimmed.to_string())
}

// Validate Datetime
pub fn validate_date(date_str: &str) -> Result<NaiveDate, ExpenseError> {
    let date = NaiveDate::parse_from_str(date_str.trim(), crate::config::DATE_FORMAT)?;

    let today = chrono::Local::now().date_naive();
    let one_year_ago = today - chrono::Duration::days(365);
    let one_year_ahead = today + chrono::Duration::days(365);

    if date < one_year_ago || date > one_year_ahead {
        return Err(ExpenseError::ParseError(
            "วันที่ต้องอยู่ในช่วง 1 ปีที่ผ่านมาถึง 1 ปีข้างหน้า".to_string(),
        ));
    }

    Ok(date)
}

// Validate Type
pub fn parse_entry_type(type_str: &str) -> Result<crate::models::EntryType, ExpenseError> {
    match type_str.trim().to_lowercase().as_str() {
        "income" | "รายรับ" => Ok(crate::models::EntryType::Income),
        "expense" | "รายจ่าย" => Ok(crate::models::EntryType::Expense),
        _ => Err(ExpenseError::InvalidEntryType(type_str.to_string())),
    }
}

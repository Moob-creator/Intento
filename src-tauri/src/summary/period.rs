use chrono::{Datelike, Duration, NaiveDate, TimeZone, Utc};

/// Period calculator for summary generation
pub struct PeriodCalculator;

impl PeriodCalculator {
    /// Get today's start and end timestamps
    pub fn today() -> (i64, i64) {
        let now = Utc::now();
        let start = now
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();
        let end = start + 86400 - 1; // 23:59:59
        (start, end)
    }

    /// Get this week's start and end timestamps (Monday to Sunday)
    pub fn this_week() -> (i64, i64) {
        let now = Utc::now();
        let weekday = now.weekday().num_days_from_monday();

        let start = (now - Duration::days(weekday as i64))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = start + 7 * 86400 - 1;
        (start, end)
    }

    /// Get this month's start and end timestamps
    pub fn this_month() -> (i64, i64) {
        let now = Utc::now();

        let start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let next_month = if now.month() == 12 {
            NaiveDate::from_ymd_opt(now.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(now.year(), now.month() + 1, 1).unwrap()
        };

        let end = next_month
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// Get this semi-annual period's start and end timestamps
    pub fn this_semi_annual() -> (i64, i64) {
        let now = Utc::now();
        let year = now.year();

        let (start_month, end_month) = if now.month() <= 6 {
            (1, 6) // Jan-Jun
        } else {
            (7, 12) // Jul-Dec
        };

        let start = NaiveDate::from_ymd_opt(year, start_month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end_date = if end_month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(year, end_month + 1, 1).unwrap()
        };

        let end = end_date
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// Get this year's start and end timestamps
    pub fn this_year() -> (i64, i64) {
        let now = Utc::now();
        let year = now.year();

        let start = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp() - 1;

        (start, end)
    }

    /// Get yesterday's start and end timestamps
    pub fn yesterday() -> (i64, i64) {
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let start = yesterday
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();
        let end = start + 86400 - 1;
        (start, end)
    }

    /// Get last week's start and end timestamps
    pub fn last_week() -> (i64, i64) {
        let (this_week_start, _) = Self::this_week();
        let last_week_start = this_week_start - 7 * 86400;
        let last_week_end = this_week_start - 1;
        (last_week_start, last_week_end)
    }

    /// Get last month's start and end timestamps
    pub fn last_month() -> (i64, i64) {
        let now = Utc::now();

        let (year, month) = if now.month() == 1 {
            (now.year() - 1, 12)
        } else {
            (now.year(), now.month() - 1)
        };

        let start = NaiveDate::from_ymd_opt(year, month, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let this_month_start = NaiveDate::from_ymd_opt(now.year(), now.month(), 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap()
            .timestamp();

        let end = this_month_start - 1;

        (start, end)
    }

    /// Get period for a specific summary type
    pub fn current_period(summary_type: &crate::db::models::SummaryType) -> (i64, i64) {
        use crate::db::models::SummaryType;
        match summary_type {
            SummaryType::Daily => Self::today(),
            SummaryType::Weekly => Self::this_week(),
            SummaryType::Monthly => Self::this_month(),
            SummaryType::SemiAnnual => Self::this_semi_annual(),
            SummaryType::Yearly => Self::this_year(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_today() {
        let (start, end) = PeriodCalculator::today();
        assert!(start < end);
        assert_eq!(end - start, 86399); // 24h - 1s
    }

    #[test]
    fn test_this_week() {
        let (start, end) = PeriodCalculator::this_week();
        assert!(start < end);
        assert_eq!(end - start, 7 * 86400 - 1);
    }

    #[test]
    fn test_this_month() {
        let (start, end) = PeriodCalculator::this_month();
        assert!(start < end);
        let days = (end - start + 1) / 86400;
        assert!(days >= 28 && days <= 31);
    }
}

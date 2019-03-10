pub fn is_leap_year(year: u64) -> bool {

    let mut is_leap = false;

    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                is_leap = true;
            } else {
                is_leap = false;
            }
        } else {
            is_leap = true;
        }
    } else {
        is_leap = false;
    }

    is_leap
}

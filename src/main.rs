fn main() {
    let year = 2000;
    let is_leap = leap::is_leap_year(year);
    println!("{} is a leap year? {}", year, is_leap);
}

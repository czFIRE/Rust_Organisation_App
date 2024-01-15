pub fn check_email_validity(email: String) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9. _-]+@[a-zA-Z0-9. -]+\. [a-zA-Z]{2,4}$")
        .expect("Should be valid.");
    let email_captures = email_regex.captures(email.as_str());
    email_captures.is_some()
}

pub fn check_phone_validity(phone: String) -> bool {
    let phone_regex = Regex::new(r"^[\+]?[(]?[0-9]{3}[)]?[-\s\.]?[0-9]{3}[-\s\.]?[0-9]{4,6}$")
        .expect("Should be valid.");
    let phone_captures = phone_regex.captures(phone.as_str());
    phone_captures.is_some()
}

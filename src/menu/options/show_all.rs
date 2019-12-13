use crate::Status;


pub fn exec(mut status: Status, input: String) -> Status {

    let amt = input.trim_start_matches("a").trim();
    let show_amount: i32;

    match amt.parse::<i32>() {
        Ok(n) => show_amount = n,
        Err(_e) => show_amount = if status.show_all != -1 { -1 } else { 0 }
    }

    if show_amount > -3 {
        status.show_all = show_amount
    }

    return status;
}
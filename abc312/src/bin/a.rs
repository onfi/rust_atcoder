use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    };
    let result = match s.as_str() {
        "ACE" => "Yes",
        "BDF" => "Yes",
        "CEG" => "Yes",
        "DFA" => "Yes",
        "EGB" => "Yes",
        "FAC" => "Yes",
        "GBD" => "Yes",
        _ => "No",
    };
    println!("{}", result);
}

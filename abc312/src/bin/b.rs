use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n]
    };
    const TAK_SIZE: usize = 9;

    for i in 0..=n - TAK_SIZE {
        for j in 0..=m - TAK_SIZE {
            if "###." == &s[i + 0][j..=j + 3]
                && "###." == &s[i + 1][j..=j + 3]
                && "###." == &s[i + 2][j..=j + 3]
                && "...." == &s[i + 3][j..=j + 3]
                && "...." == &s[i + 5][j + 5..=j + 5 + 3]
                && ".###" == &s[i + 6][j + 5..=j + 5 + 3]
                && ".###" == &s[i + 7][j + 5..=j + 5 + 3]
                && ".###" == &s[i + 8][j + 5..=j + 5 + 3]
            {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}

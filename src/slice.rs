
pub fn slice_test(){

    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part = &s[5..9];
    println!("{}={}+{}",s,part1,part);

}

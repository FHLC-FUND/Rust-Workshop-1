use std::io;
use std::fs::File;
use std::io::Read;
fn main(){
    //Case 1
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [1,8,10];
    let mut index_org = 0;
    let mut result = false;
    while index_org < org_arr.len() {
        let mut index_sub=0;
        if org_arr[index_org]==sub_arr[index_sub]{
            let mut i = index_org;
            while i < org_arr.len()&&index_sub<sub_arr.len(){
                if org_arr[i]==sub_arr[index_sub] {
                    result = true;
                    i+=1;
                    index_sub+=1;
                }else{
                    index_sub=0;
                    result = false;
                    break;
                }
            }
        } if result { break } index_org += 1;
    } if result == true{ println!("Là chuỗi con") } else { println!("Không phải chuỗi con") }
    //Case 2
    let mut input = String::new();
        println!("Nhập kí tự bất kì: ");
        io::stdin().read_line(&mut input).expect("Không đọc được");
        input.pop();
    let mut file = File::open("./text.txt").expect("Không thấy file!");
        let mut text = String::new();
        file.read_to_string(&mut text).expect("Lỗi khi đọc file!");
    let first_char = input.chars().next().unwrap().to_lowercase().next().unwrap();
    let chars = text.chars().collect::<Vec<char>>();
    let input_chars = input.chars().collect::<Vec<char>>();
    let mut index = 0;
    let mut count = 0;
    loop {
        if index >= text.len() {
            break;
        }
        else{
            if chars[index].to_lowercase().next().unwrap() == first_char { 
                if index + input.len() > text.len() {
                    println!("Không hợp lệ");
                    break;
                }
                let mut check = true;
                for i in 1..input.len() {
                    if chars[index + i] != input_chars[i] { check = false }
                }
                if check {
                    index = index + input.len();
                    count += 1;
                }
            }
        } index += 1;
    }
    println!("Số lượng: {}", count);
}

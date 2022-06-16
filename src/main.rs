use std::fs;
use std::io;
fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [1,8,10];
    let mut index_org = 0;
    let mut result = false;
    if sub_arr.len() == 0 || org_arr.len() == 0 {
        println!("Chuỗi cần so sánh = 0")
    }
    else if sub_arr > org_arr{
        println!("Chuỗi con lớn hơn chuỗi bố")
    }
    else{
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
            }
            if result {
                break;
            }
            index_org += 1;
        }
        if result == true{
            println!("Là chuỗi con");
        }else {
            println!("Không phải chuỗi con")
        }
    }
    //
    let mut line = String::new();
        println!("Nhập kí tự cần đếm trong văn bản: ");
        io::stdin().read_line(&mut line).unwrap();
        let chars: Vec<_> = line.chars().collect();

    let mut data = fs::read_to_string("./Text").expect("Unable to read file");
    let counting = data.chars().filter(|&c| c == chars).count();
    println!("{}", counting);
}

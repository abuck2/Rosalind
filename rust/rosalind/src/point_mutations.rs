use std::fs;
//use std::collections::btree_map::BTreeMap;
use std::io;
use std::io::Write;

fn main() {
    let filename = "rosalind_rna.txt";
    let content = file_opener(filename.to_string());


}


pub fn file_opener(filename:String) -> String {
    //let filename = "C:\\Users\\Alexis\\Documents\\a imprimer\\test\\rosalind_ini5.txt";
    let contents = fs::read_to_string(filename).expect("Unable to read file");
    io::stdout().flush().unwrap(); 
    println!("{}", contents);

    //multiple lines
    let mut content_split :Vec<&str> = contents.split('\n').collect();

    let mut i :i64 = 1;
    for line in &mut content_split {
        if i%2==0{
            println!("{}", line);
        }
        i += 1;
    }
    contents
}

pub fn HammingDistance(content:String){




}

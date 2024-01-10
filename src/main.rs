use std::char;
use std::io;
use std::fs;

struct Pair<'a> {
    name: &'a str,
    pass : &'a str,
}

fn add( tgt : & mut String, x : char, count : u32){
    let mut added_string :String = "".to_string();
    
    for i in 0..count {
        added_string.push(x);
    };

    tgt.push_str(added_string.as_str());

}


fn display(pairs : Vec<Pair>){

    let width : u8 = 120;
    let name_width : u8 = 30;
   let pass_width : u8 = width-name_width;

    

    
    for p in pairs {
        let mut line : String = "".to_string();
        line.push('|');
        let name_padding : f64 = (name_width as f64 - p.name.len() as f64) /2.0;
        add(& mut line , ' ', name_padding.floor() as u32);
        line.push_str(p.name);
        add(& mut line, ' ', name_padding.ceil() as u32);
        line.push('|');
        let pass_padding : f64 = (pass_width as f64 - p.pass.len() as f64) / 2.0;
        add(& mut line, ' ', pass_padding.floor() as u32);
        line.push_str(p.pass);
        add(& mut line,' ', pass_padding.ceil() as u32);
        line.push('|');
        println!(" {}", line);
    }
}

fn main() {
    let file_path :String = String::from("C:\\Users\\marcu\\Desktop\\items\\RustLang\\passwordMngr\\test01\\data\\usrdata.txt");


    println!("Password manager v: alpha.0.1");

    println!("\nPlease enter a username.");

    let mut usrname : String = String::new();

    io::stdin().read_line(&mut usrname).expect("failed to read line");

    println!("Hello {}", usrname);

    let file_content = fs::read_to_string(file_path).expect("error reading the file");

    println!("file contents: {}", file_content);

    let mut pairs : Vec<Pair> = Vec::new();

    let unsplitted_pairs : Vec<&str> = file_content.split('\n').collect();

    for p in unsplitted_pairs {
        println!("p: \n {}", p);
        let spl : Vec<&str> = p.split(',').collect();
        pairs.push(Pair{name : spl[0], pass : spl[1]})
    }

    display(pairs);


}

use std::collections::HashMap;
use std::fs;
use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::process::exit;

fn main() -> Result<(), Box<dyn Error>> {
    let mut dict: HashMap<String,i32> = HashMap::new();
    let mut label_dict: HashMap<String,i32> = HashMap::new();
//    dict.insert("key1", "value1");
    let mut filepath = "file.txt";
    //read args
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1{
        filepath = &args[1];
    }
    //check if file exists
    if !std::path::Path::new(filepath).exists(){
        println!("Error: File does not exist");
        exit(1);
    }

    let file = fs::File::open(filepath)?;
    let reader = BufReader::new(file);
    //to string array
    let mut i : usize = 0;
    //for line in reader.lines() 
    let lines: Vec<_> = reader.lines().collect();
    if lines.len() == 0{
        println!("Error: File is empty");
        exit(1);
    }
    let linecount = lines.len();
    //get labels
    while i<linecount
    {
        i+=1;
        let line = lines.get(i-1).unwrap().as_ref().unwrap().trim();
        if line.is_empty(){
            continue;
        }
        let split_lines: Vec<&str> = line.split(":").collect();
        if split_lines[0]=="prints" || split_lines[0]=="printsc"{
            continue;
        }
        //if there is a space in the line, then exit, because i just dont like it.
        for c in line.chars(){
            //print!("{}", c);
            if c == '#'{
                break;
            }
            if c == ' '{
                println!("Error: There is a space in the line {}.", i);
                println!("Thats pretty cringe ngl");
                exit(1);
            }
        }
        
        if split_lines[0] == "label"{
            label_dict.insert(split_lines[1].to_string(), i as i32);
        }
    }
    i = 0;
    while i<linecount
    {
        i+=1;
        let line = lines.get(i-1).unwrap().as_ref().unwrap().trim();
        if line.is_empty(){
            continue;
        }   
        let split_lines: Vec<&str> = line.split(":").collect();
        if split_lines[0] == "var"{
            dict.insert(split_lines[1].to_string(), split_lines[2].parse::<i32>().unwrap());
        }
        if split_lines[0] == "print"{
            println!("{}", get_value(split_lines[1].to_string(), dict.clone()));
        }
        if split_lines[0] == "printc"{
            print!("{}", get_value(split_lines[1].to_string(), dict.clone()));
        }
        if split_lines[0] == "prints"{
            println!("{}", split_lines[1].to_string())
        }
        if split_lines[0] == "printsc"{
            print!("{}", split_lines[1].to_string())
        }
        if split_lines[0] == "add"{
            let a = get_value(split_lines[1].to_string(), dict.clone());
            let b = get_value(split_lines[2].to_string(), dict.clone());
            let sum = a + b;
            if sum<0{
                println!("Error: The sum is negative");
                println!("Because of the risk of owoflows, this is not supported");
                exit(1);
            }
            //write sum into a
            dict.insert(split_lines[1].to_string(), sum);
        }
        if split_lines[0] == "sub"{
            let a = get_value(split_lines[1].to_string(), dict.clone());
            let b = get_value(split_lines[2].to_string(), dict.clone());
            let sum = a - b;
            if sum<0{
                println!("Error: The sum is negative");
                println!("Because of the risk of owoflows, this is not supported");
                exit(1);
            }
            //write sum into a
            dict.insert(split_lines[1].to_string(), sum);
        }
        if split_lines[0] == "mul"{
            let a = get_value(split_lines[1].to_string(), dict.clone());
            let b = get_value(split_lines[2].to_string(), dict.clone());
            let sum:i32 = a * b;
            if sum<0{
                println!("Error: The sum is negative");
                println!("Because of the risk of owoflows, this is not supported");
                exit(1);
            }
            //write sum into a
            dict.insert(split_lines[1].to_string(), sum);
        }
        if split_lines[0] == "div"{
            let a = get_value(split_lines[1].to_string(), dict.clone());
            let b = get_value(split_lines[2].to_string(), dict.clone());
            let sum = a / b;
            if sum<0{
                println!("Error: The sum is negative");
                println!("Because of the risk of owoflows, this is not supported");
                exit(1);
            }
            //write sum into a
            dict.insert(split_lines[1].to_string(), sum);
        }
        if split_lines[0] == "go"{
            //go:69
            //go:69:if:a:<:b
            
            if split_lines.len()>2{
                if compare(split_lines[3].to_string()+":"+&split_lines[4].to_string()+":"+&split_lines[5].to_string(), dict.clone()){
                    i = get_value(split_lines[1].to_string(), label_dict.clone()) as usize;
                }
            }
            else{
                i = get_value(split_lines[1].to_string(), label_dict.clone()) as usize;
            }

        }
        if split_lines[0] == "goto"{
            i = get_value(split_lines[1].to_string(), label_dict.clone()) as usize;

        }
        if split_lines[0] == "goif"{
            //goif:1:>:2:3
            if compare(split_lines[1].to_string()+":"+&split_lines[2].to_string()+":"+&split_lines[3].to_string(), dict.clone()){
                i = get_value(split_lines[4].to_string(), label_dict.clone()) as usize;
            }
        }
    }
    Ok(())
}
fn get_value(string: String, dict: HashMap<String,i32>) -> i32{
    //if the string is a number, then return it, else return the value of the variable
    if string.parse::<i32>().is_ok(){
        return string.parse::<i32>().unwrap();
    }
    else{
        if !dict.contains_key(&string){
            println!("Error: Variable {} does not exist", string);
            exit(1);
        }	
        return dict.get(&string).unwrap().clone();
    }
}


fn compare(string: String, dict: HashMap<String,i32>) -> bool{
    //e.g. 1:>:2 or uwu:<:owo
    let split_lines: Vec<&str> = string.split(":").collect();
    let a = get_value(split_lines[0].to_string(), dict.clone());
    let b = get_value(split_lines[2].to_string(), dict.clone());
    if split_lines[1] == ">"{
        return a>b;
    }
    if split_lines[1] == "<"{
        return a<b;
    }
    if split_lines[1] == "="{
        return a==b;
    }
    return false;

}
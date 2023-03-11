use std::io;

fn main(){
    let mut height: String = String::new();
    let mut weight: String = String::new();

    println!("enter your height (on meter)");
    io::stdin().read_line(&mut height).expect("failed to read height");

    println!("enter your weight");
    io::stdin().read_line(&mut weight).expect("failed to read weight");

    let height_int: f32 = height.trim().parse::<f32>().expect("failer parse height");
    let weight_int: f32 = weight.trim().parse::<f32>().expect("failer parse weight");

    let bmi_calculator:f32 = weight_int/(height_int * height_int);

    if bmi_calculator < 18.5{
        println!("you are thin");
    }else if bmi_calculator >= 18.5 && bmi_calculator <= 24.8{
        println!("you are normal");
    }else if bmi_calculator >= 25.0 && bmi_calculator <= 29.9{
        println!("you are overheight");
    }else {
        println!("you are obese")
    }

    println!("bmi scors {}", bmi_calculator);


}

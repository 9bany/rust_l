fn main() {
    let my_variable = 0;
    const MY_CONSTANT: u8 = 0;
    static MY_STATIC: u8 = 0;

    let mut my_mutable_variable = 0;
    
    
    owner_ship();
}

fn this_is_fn() -> u8 {
    return 2;
}

fn this_fn_is_not_return() -> () {
    this_is_fn();
}

fn return_fn() -> u8 {
    4
}

fn return_fn_a(a: u32) -> i32 {
    // casting type of a
    a as i32
}

fn string_basic() {
    let my_string: String = String:: from("hello world!!"); // -> allocation on heap
    let my_str: &str = "hello work"; // -> allocation on stack
    let my_str_2: &str = &my_string[0..9];

    let my_arr: [u32; 5] = [1,2,3,4,5];
    let my_arr2: [usize; 3] = [1,2,3];

    let slice: &[usize] = &my_arr2[0..2];

    let collection_of_chars : &str = my_str.chars().as_str();
}

fn numbers() {
    let mut num1: u8 = 9;
    let mut num2: i8 = 9;
    let mut flot: f32 = 9.0;

}

fn structed() {
    struct MyStruct {
        feild: u8,
    }

    let s: MyStruct = MyStruct { feild: 3 };
}

fn enum_d() {
    enum ErrorEnum{
        BrainTooTired,
        TimeOfDay(String),
        CoffeeCupEmpty,
    }

    fn work() -> Result<(), ErrorEnum> {
        let state: &str = "";
        if state == "missing semi-colon" {
            Err(ErrorEnum::BrainTooTired)
        } else{
            Ok(())
        }
    }
}

fn marco() {
    let num: i8 = 2;
    print!("ok {}", num)
}

fn panic_error() {
    let an_error = true;

    if an_error {
        panic!("this is an error")
    }
}

fn owner_ship() {
    let first_string: String = String::from("value");
    let second_string: &String = &first_string;
    println!("this is string: {}", first_string);
}
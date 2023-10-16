

struct Person {
    first_name: String,
    last_name: String
}


impl Person {

    fn new(first: &str, name: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: name.to_string()
        }
    }

}

impl Person {
    fn full_name(&self) -> String {
        return format!{"{} {}", self.first_name, self.last_name}
    }
}

// normal func
fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

// mut func
fn mut_num(a: &mut i64) {
    *a = 5
}

// func ref

fn ref_func(a: &i64) -> i64 {
    *a + 1
}

fn basic() {
    let mut answer = 0;
    println!("Hello {}", answer);

    for i in 0..5 {
        answer += i;
    }
    println!("Hello {}", answer);

    println!("sum of 4 and 5 is  {}", sum(4, 5));

    let mut a = 2;
    println!("first number of the a: {}", a);
    mut_num(&mut a);
    println!("the value of a before change: {}", a);
    let new_value = ref_func(&a);
    println!("the value of a before plus 1: {}", new_value);

    let x_pi = 2.0 * std::f64::consts::PI;
    println!("the value of double PI: {}", x_pi);
}

fn arr() {

    let a = [1,2,3,4];
    for i in a {
        println!("{}", i);
    }
}

fn main() {
    // basic();
    arr();
    
    let p = Person::new("John","Smith");
    println!("person {} ", p.full_name());
}
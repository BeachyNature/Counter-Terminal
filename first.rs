// Public enumerations to define a list
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}   

fn print_list<T: std::fmt::Debug>(list: &List<T>){
    match list {
        List::Cons(value, next) => {
            print!("{:?} ", value);
            print_list(next);
        }
        List::Nil => {}
    }
}

pub fn main() { 
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    print_list(&list);
}


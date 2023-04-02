#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generic_data_types_ex() {
    println!("\n====== Generic data types ======");

    println!("******Largest of a list using generic funcion");
    largest_of_list();

    println!("******Structure using generics");
    generic_struct();

    println!("******Generic methods");
    generics_in_method_definition();

    println!("******Generic mix - Structure and methods defintion");
    generics_in_struct_and_method_mix();
}

fn largest_of_list() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!(
        "Integer struct: {:?}, Floating point struct: {:?}",
        integer.x, float.y
    );

    let more_than_one_generic_type_struct = Point2 { x: 8, y: 10.3 };
    println!(
        "More than one generic type struct: {:?} {:?}",
        more_than_one_generic_type_struct.x, more_than_one_generic_type_struct.y
    )
}

fn generics_in_method_definition() {
    let p = Point { x: 5, y: 10 };

    println!("Generic in method definition - p.x = {}", p.x());

    let genrics_with_constraint_types = Point {
        x: 5.7 as f32,
        y: 10.2 as f32,
    };

    println!(
        "Generic in method definition - constraint on type argument  = {}",
        genrics_with_constraint_types.distance_from_origin()
    );
}


fn generics_in_struct_and_method_mix(){
    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
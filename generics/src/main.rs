fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_generics(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_generics(&char_list);
    println!("The largest char is {}", result);

    println!("Struct Generics");
    let integer = Point { x: 5, y: 100};
    let _float = Point { x: 1.0, y: 4.0 };
    println!("The integer x is {}", integer.x);
    println!("The integer y is {}", integer.y);
    let _point_multi = PointMulti {x: 1.0, y: 10};
    let _both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 4.0 as f32, y: 4.3 as f32 };
    let _integer_and_float = PointMulti { x: 5, y: 4.0 };
    println!("The point-multi generics method x is {}", _integer_and_float.x);
    println!("The point-multi generics method y is {}", _integer_and_float.y);
    println!("The point generics method is {}", both_float.something());
    println!("The point distance from origin is {}", both_float.distance_from_origin());
    println!("The point generics field is {}", both_float.y);
    println!("MIXUP");
    let p1 = PointMixUp { x: 5, y: 10.4 };
    let p2 = PointMixUp { x: "Hello", y: 'c' };
    let p3 = p1.mix(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_generics<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn something(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointMulti<T, U> {
    x: T,
    y: U
}

struct PointMixUp<X1, Y1> {
    x: X1,
    y: Y1
}

impl<X1, Y1> PointMixUp<X1, Y1> {
    fn mix<X2,Y2>(self, other: PointMixUp<X2,Y2>) -> PointMixUp<X1, Y2> {
        PointMixUp {
            x: self.x,
            y: other.y
        }
    }
}

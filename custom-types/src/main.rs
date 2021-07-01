#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn calculate_area(rect: Rectangle) {
    println!("x points: {} {}", rect.top_left.x, rect.bottom_right.x);
    println!("y points: {} {}", rect.top_left.y, rect.bottom_right.y);
    let width = rect.bottom_right.x - rect.top_left.x;
    let height = rect.top_left.y - rect.bottom_right.y;
    println!("width: {}", width.abs());
    println!("height: {}", height.abs());
    let area = width.abs() * height.abs();
    println!("area: {}", area);
}

fn square(point: Point, dimension: f32) -> Rectangle {
    // point is bottom left of square
    Rectangle {
        top_left: Point {
            x: point.x,
            y: dimension,
        },
        bottom_right: Point {
            x: dimension,
            y: point.y,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: top_edge,
        y: left_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let sq = square(point, 3.0);
    calculate_area(sq);
}

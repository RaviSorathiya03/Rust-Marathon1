enum Shapes{
    Circle(f64),
    Rectangle(f64, f64)
}

fn main(){
    let my_shape = Shapes::Circle(5.0);
    let area = calculate_area(my_shape);
    println!("{}", area);
}

fn calculate_area(shape: Shapes) -> f64{
    let area= match shape{
        Shapes::Rectangle(a, b) => a*b,
        Shapes::Circle(a) => 3.14 * a * a 
    };

    return area;
}

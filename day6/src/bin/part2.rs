fn calc_boundary_charge_times<T: Into<f64> + From<f64>>(tx: T, dx: T) -> (T, T) {
    let tx = tx.into();
    let dx = dx.into();

    let a = 1.0;
    let b = -tx;
    let c = dx;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        panic!("No real roots");
    }
    let sqrt_discriminant = discriminant.sqrt();
    let root1 = (-b - sqrt_discriminant) / (2.0 * a);
    let root2 = (-b + sqrt_discriminant) / (2.0 * a);

    (root1.ceil().into(), root2.floor().into())
}

fn main() {
    println!("Part 2!");
		let (time, distance): (u64, u64) = (48989083, 390110311121360);

    let (min_charge_time, max_charge_time) =
        calc_boundary_charge_times(time as f64, distance as f64);

    println!("Result: {}", (max_charge_time - min_charge_time) + 1.0);
}

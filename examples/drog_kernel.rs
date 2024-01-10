use rust_for_multimedia_2023::drog::kernel;

fn main() {
    let (x_kernel, y_kernel) = kernel(3, 0.5);

    println!("X Kernel: \n{:?}", x_kernel);
    println!("Y Kernel: \n{:?}", y_kernel);
}
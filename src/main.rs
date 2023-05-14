#[derive(Debug)]
struct MyOptions {
    target_path: String,
}

fn main() {
    let mut installer = binstaller::GraphicalInstaller::<MyOptions>::default();

    let mut frame1 = binstaller::frame::GraphicalInstallerFrame::default();
    frame1.set_executor(&frame1_function).unwrap();
    installer.add_frame(frame1).unwrap();

    let mut frame2 = binstaller::frame::GraphicalInstallerFrame::default();
    frame2.set_executor(&frame2_function).unwrap();
    installer.add_frame(frame2).unwrap();

    let mut frame3 = binstaller::frame::GraphicalInstallerFrame::default();
    frame3.set_executor(&frame3_function).unwrap();
    installer.add_frame(frame3).unwrap();

    installer.register_data(MyOptions {
        target_path: String::from("D:\\Dev\\Rust\\projects\\binstaller"),
    });

    let my_options = installer.retreive_data().unwrap();

    println!("Options: {my_options:?}");
    // Prints 'Options: MyOptions { target_path: "D:\\Dev\\Rust\\projects\\binstaller" }'

    installer.run().unwrap();
}

fn frame1_function(data: Option<&mut MyOptions>) {
    println!("Salut, i have some data: {data:?}");
}

fn frame2_function(data_opt: Option<&mut MyOptions>) {
    println!("Modifying data ...");

    if let Some(data) = data_opt {
        data.target_path = String::from("Modified data")
    }
}

fn frame3_function(data: Option<&mut MyOptions>) {
    println!("Salut, i have some Modified data: {data:?}");
}

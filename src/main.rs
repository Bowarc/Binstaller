#[derive(Debug)]
struct MyOptions {
    name: String,
    age: usize,
}

fn main() {
    let mut installer = binstaller::GraphicalInstaller::<MyOptions>::default();

    installer
        .add_frame(binstaller::frame::GraphicalInstallerFrame::default())
        .unwrap();

    installer.register_data(MyOptions {
        name: "Yes".to_string(),
        age: 69,
    });

    let my_options = installer.retreive_data().unwrap();

    println!("Options: {my_options:?}")
    // installer.register_data("data1", 69).unwrap();

    // let value = installer.retreive_data("data1").unwrap();

    // let my_value: usize = value.into();
}

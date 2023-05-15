#[derive(Debug)]
struct MyOptions {
    downloader: binstaller::modules::downloader::DownloaderPool,
}

#[tokio::main]
async fn main() {
    let mut installer = binstaller::GraphicalInstaller::<MyOptions>::default();
    installer.register_data(MyOptions::default());

    let mut frame1 = binstaller::frame::GraphicalInstallerFrame::default();
    frame1
        .set_executor(&|ui, data| {
            ui.label(format!("Hey, i have some data: {data:#?}"));
        })
        .unwrap();
    installer.add_frame(frame1).unwrap();

    let mut frame2 = binstaller::frame::GraphicalInstallerFrame::default();
    frame2.set_executor(&frame2_function).unwrap();
    installer.add_frame(frame2).unwrap();

    let mut frame3 = binstaller::frame::GraphicalInstallerFrame::default();
    frame3.set_executor(&frame3_function).unwrap();
    installer.add_frame(frame3).unwrap();

    let mut frame4 = binstaller::frame::GraphicalInstallerFrame::default();
    frame4.set_executor(&download::<MyOptions>).unwrap();
    installer.add_frame(frame4).unwrap();

    let my_options = installer.retreive_data();

    println!("Options: {my_options:?}");
    // Prints 'Options: MyOptions { target_path: "D:\\Dev\\Rust\\projects\\binstaller" }'

    installer.run().unwrap();
}

fn frame2_function(
    ui: &mut binstaller::eframe::egui::Ui,
    data: &mut MyOptions,
) {
    // println!("Modifying data ...");
    ui.label("Target path:");
    ui.text_edit_singleline(&mut format!("{}",data.downloader.settings.download_path.display()));

    // data.target_path = String::from("Modified data")
}

fn frame3_function(
    ui: &mut binstaller::eframe::egui::Ui,
    data: &mut MyOptions,
) {
    // println!("Salut, i have some Modified data: {data:?}");

    ui.label(format!("The options have been modified with: {data:#?}"));
}

// This is full of hard coded shit, but 1 im fcking tired, 2 it's more like a proof of concept, to see if i like it or not

// working
// so, next step is maybe to make modules (on lib) for the user to add to their data struct to be used later
// like this download module could regroup:
//      a list of pair (url, file name) to dl
//      some settings
//      2 or 3 functions for ui and fetching from / starting downloaders  (separating ui and functionalities so the user can make their own ui)
fn download<Data: Default + std::fmt::Debug>(
    ui: &mut binstaller::eframe::egui::Ui,
    data: &mut MyOptions,
) {
    data.downloader.update();
    data.downloader.ui(ui)
}


impl Default for MyOptions{
    fn default() -> Self {
        MyOptions {
            downloader: binstaller::modules::downloader::DownloaderPool{
                settings: binstaller::modules::downloader::DownloaderSettings{
                    download_path: std::path::PathBuf::from("D:/dev/rust/projects/binstaller/downloads/"),
                    concurent_download_limit: None,
                },
                requests: vec![
                    binstaller::modules::downloader::DownloadRequest {
                        file_name: String::from("lumin_mpv.exe"),
                        url: String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_mpv.exe"),
                        path: None
                    },
                    binstaller::modules::downloader::DownloadRequest {
                        file_name: String::from("lumin_client.exe"),
                        url: String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_client.exe"),
                        path: None
                    },
                    binstaller::modules::downloader::DownloadRequest {
                        file_name: String::from("lumin_daemon.exe"),
                        url: String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_daemon.exe"),
                        path: None
                    },
                    binstaller::modules::downloader::DownloadRequest {
                        file_name: String::from("readme.exe"),
                        url: String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/README.exe"),
                        path: None
                    },
                ],
                list: vec![],
            },

        }
    }
}
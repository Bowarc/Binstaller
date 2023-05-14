#[derive(Debug, Default)]
struct MyOptions {
    asset_list: Vec<String>,
    downloaded_assets: Vec<String>,

    target_path: String,
}

#[tokio::main]
async fn main() {
    let mut installer = binstaller::GraphicalInstaller::<MyOptions>::default();
    installer.register_data(MyOptions {
        url: vec![
            String::from(
                "https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_client.exe",
            ),
            String::from(
                "https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_daemon.exe",
            ),
            String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/lumin_mpv.exe"),
            String::from("https://github.com/Bowarc/Lumin/releases/download/0.1.3/README.txt"),
        ],
        downloaded_assets: vec![],
        target_path: String::from("D:\\Dev\\Rust\\projects\\binstaller\\test_downloads\\"),
    });

    let mut frame1 = binstaller::frame::GraphicalInstallerFrame::default();
    frame1
        .set_executor(&|ui, data, _| {
            ui.label(format!("Hey, i have some data: {data:?}"));
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
    downloader_pool: &mut binstaller::downloader::DownloaderPool,
) {
    // println!("Modifying data ...");
    ui.label("Target path:");
    ui.text_edit_singleline(&mut data.target_path);

    // data.target_path = String::from("Modified data")
}

fn frame3_function(
    ui: &mut binstaller::eframe::egui::Ui,
    data: &mut MyOptions,
    downloader_pool: &mut binstaller::downloader::DownloaderPool,
) {
    // println!("Salut, i have some Modified data: {data:?}");

    ui.label(format!("The options have been modified with: {data:#?}"));
}

// This is full of hard coded shit, but 1 im fcking tired, 2 it's more like a proof of concept, to see if i like it or not
fn download<Data: Default + std::fmt::Debug>(
    ui: &mut binstaller::eframe::egui::Ui,
    data: &mut MyOptions,
    downloader_pool: &mut binstaller::downloader::DownloaderPool,
) {
    // println!("Salut, i have some Modified data: {data:?}");

    let dl_limit = if let Some(limit) = downloader_pool.settings.concurent_download_limit {
        limit
    } else {
        usize::MAX
    };

    if downloader_pool.active_download_count() < dl_limit {


        downloader_pool.start_download(
            std::path::PathBuf::from(data.target_path.clone()),
            data.asset_list.get(0).unwrap().to_string(),
            "lumin_client.exe".to_string(),
        )
    }

    ui.separator();
    for download in downloader_pool.list.iter_mut() {
        ui.label(format!(" Lumin {:.1}%", download.prcentage));
        ui.separator();

        match download.update_receiver.try_recv() {
            Ok(msg) => match msg {
                binstaller::downloader::UpdateMessage::Starting => println!("Stating download"),
                binstaller::downloader::UpdateMessage::Prcent(value) => {
                    download.prcentage = value;
                    println!("Prcentage update: {value}")
                }
                binstaller::downloader::UpdateMessage::Done => {
                    download.done= true;
                    println!("Download ended")
                },
                binstaller::downloader::UpdateMessage::Error(e) => {
                    eprintln!("Got an error while using the downloader: {e}")
                }
            },
            Err(e) =>{

                 // eprintln!("{e}")
            },
        }
    }

    // ui.label(format!("The options have been modified with: {data:#?}"));
}

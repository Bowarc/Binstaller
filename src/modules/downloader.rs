pub enum UpdateMessage {
    Starting,
    Prcent(f32),
    Done,
    Error(crate::error::DownloaderError),
}

#[derive(Debug)]
pub struct DownloaderPool {
    pub settings: DownloaderSettings,
    pub requests: Vec<DownloadRequest>,

    pub list: Vec<Downloader>,
}

#[derive(Debug)]
pub struct DownloadRequest{
    pub file_name: String,
    pub url: String,
    /// Be carefull, if you set a path for this and for DownloaderSettings::download_path
    /// This path will be added to DownloaderSettings::download_path
    /// Example:
    ///     DownloaderSettings::download_path = D:/MyApp/
    ///     DownloadRequest::path = updater/
    /// The file will be downloaded to the folder D:/MyApp/updater/
    pub path: Option<std::path::PathBuf>, 
}

#[derive(Debug)]
pub struct DownloaderSettings {
    pub download_path: std::path::PathBuf,
    pub concurent_download_limit: Option<usize>,
}

#[derive(Debug)]
pub struct Downloader {
    file_name: String,
    target_directory: std::path::PathBuf,
    target_url: String,
    pub update_receiver: std::sync::mpsc::Receiver<UpdateMessage>,

    pub prcentage: f32,
    pub done: bool,
    // thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl DownloaderPool {
    pub fn start_download(
        &self,
        target_directory: std::path::PathBuf,
        target_url: String,
        file_name: String,

    ) -> std::sync::mpsc::Receiver::<UpdateMessage> {


        let (sender, receiver) = std::sync::mpsc::channel::<UpdateMessage>();
        sender.send(UpdateMessage::Starting).unwrap();
        let handle = tokio::task::spawn(async  {
            download(target_url, target_directory, file_name, sender).await
        });
        receiver

    }


    pub fn update(&mut self){
        let dl_limit = if let Some(limit) = self.settings.concurent_download_limit {
            limit
        } else {
            usize::MAX
        };


        'start_check: for download_request in self.requests.iter(){

            if self.list.len() < dl_limit {
                let target_directory = if let Some(custom_path) = &download_request.path{
                    self.settings.download_path.join(custom_path)
                }else{
                    self.settings.download_path.clone()
                }.clone();


                for dl in &self.list{
                    if dl.target_url == download_request.url{
                        // already started
                        continue 'start_check;
                    }
                }


                let receiver = self.start_download(target_directory.clone(), download_request.url.clone(), download_request.file_name.clone());

                self.list.push(Downloader {
                    file_name: download_request.file_name.clone(),
                    target_directory: target_directory.clone(),
                    target_url: download_request.url.clone(),
                    update_receiver: receiver,
                    prcentage: 0.0,
                    done: false,
                });
                
            }
        }


        println!("Frame");
        for download in self.list.iter_mut() {
            println!("Checking downloader for: {}", download.target_url);
            match download.update_receiver.try_recv() {
                Ok(msg) => match msg {
                    UpdateMessage::Starting => println!("Stating download"),
                    UpdateMessage::Prcent(value) => {
                        download.prcentage = value;
                        println!("Prcentage update: {value}")
                    }
                    UpdateMessage::Done => {
                        download.done= true;
                        println!("Download ended")
                    },
                    UpdateMessage::Error(e) => {
                        eprintln!("Got an error while using the downloader: {e}")
                    }
                },
                Err(e) =>{

                     eprintln!("{e}")
                },
            }
        }
    }

    pub fn ui(&mut self, ui: &mut eframe::egui::Ui){
        for download in self.list.iter_mut() {
            ui.label(format!(" {} {:.1}%", download.file_name, download.prcentage));
            ui.separator();
        }
    }
}


async fn  download(target_url: String, target_directory: std::path::PathBuf, file_name: String, sender: std::sync::mpsc::Sender<UpdateMessage>){
    use futures_util::stream::StreamExt as _;
    use std::io::Write as _;
    println!("yay");

    let reqwest_client = reqwest::ClientBuilder::new()
        .user_agent("Installer")
        .build()
        .map_err(|reason| format!("Could not create reqwest client, reason: {reason}"))
        .unwrap();

    let asset_resp =
        reqwest_client.get(target_url.clone()).send().await
            .map_err(|reason| {
                format!(
                    "Could not fetch url '{url}', reason: {reason}",
                    url = target_url
                )
            })
            .unwrap();

    let total_size = asset_resp
        .content_length()
        .ok_or(format!(
            "Could not get content length from '{}'",
            target_url.clone()
        ))
        .unwrap();

    // download chunks
    let mut file = std::fs::File::create(target_directory.join(file_name.clone()))
        .map_err(|reason| {
            format!(
                "Could not create file '{path:?}', reason: {reason}",
                path = target_directory.join(file_name.clone())
            )
        })
        .unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = asset_resp.bytes_stream();

    sender.send(UpdateMessage::Starting).unwrap();

    while let Some(item) = stream.next().await {
        let chunk = item
            .map_err(|reason| {
                format!(
                "Could not get the next chunk while downloading '{url}', reason {reason}",
                url = target_url
            )
            })
            .unwrap();
        file.write_all(&chunk)
            .map_err(|reason| {
                format!(
                    "Could not write downloaded chunk to '{path:?}', reason: {reason}",
                    path = target_directory.join(file_name.clone())
                )
            })
            .unwrap();
        let new = std::cmp::min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        sender.send(UpdateMessage::Prcent(100.*(new as f64 / total_size as f64) as f32)).unwrap()
    }
    sender.send(UpdateMessage::Done).unwrap();
}
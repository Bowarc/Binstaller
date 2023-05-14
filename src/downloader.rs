pub enum UpdateMessage {
    Starting,
    Prcent(f32),
    Done,
    Error(crate::error::DownloaderError),
}

#[derive(Default)]
pub struct DownloaderPool {
    pub settings: DownloaderSettings,

    pub list: Vec<Downloader>,
}

#[derive(Default)]
pub struct DownloaderSettings {
    pub concurent_download_limit: Option<usize>,
}

pub struct Downloader {
    target_directory: std::path::PathBuf,
    target_url: String,
    pub update_receiver: std::sync::mpsc::Receiver<UpdateMessage>,

    pub prcentage: f32,
    pub done: bool,
    // thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl DownloaderPool {
    pub fn active_download_count(&self) -> usize {
        self.list.len()
    }

    pub fn start_download(
        &mut self,
        target_directory: std::path::PathBuf,
        target_url: String,
        file_name: String,
    ) {

        for dl in &self.list{
            if dl.target_url == target_url{
                // already started
                return;
            }
        }

        let (sender, receiver) = std::sync::mpsc::channel::<UpdateMessage>();
        sender.send(UpdateMessage::Starting).unwrap();

        self.list.push(Downloader {
            target_directory: target_directory.clone(),
            target_url: target_url.clone(),
            update_receiver: receiver,
            prcentage: 0.0,
            done: false,
        });

        let handle = tokio::task::spawn(async  {
            download(target_url, target_directory, file_name, sender).await
        });
    }
}


async fn  download(target_url: String, target_directory: std::path::PathBuf, file_name: String, sender: std::sync::mpsc::Sender<UpdateMessage>){
    use futures_util::stream::StreamExt as _;
    use std::io::Write as _;

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
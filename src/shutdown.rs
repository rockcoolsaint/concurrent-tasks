use tokio::sync::oneshot;

pub fn shutdown_channel() -> (oneshot::Sender<()>, oneshot::Receiver<()>) {
    oneshot::channel::<()>()
}

pub async fn shutdown_system(shutdown_rx: oneshot::Receiver<()>) {
    shutdown_rx.await.ok();
}

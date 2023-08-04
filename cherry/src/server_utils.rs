use std::sync::Arc;

use tokio::{signal, sync::RwLock};

use crate::{db::write_date_to_disk, model::CherryData};

pub(crate) async fn shutdown_signal(data: Arc<RwLock<CherryData>>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    write_date_to_disk(&*data.read().await).unwrap();
}

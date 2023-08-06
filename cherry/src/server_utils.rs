use std::sync::Arc;

use tokio::{signal, sync::RwLock};

use crate::{db::write_date_to_disk, model::CherryData};

/// Listen for terminate or ctl + c signals and shutdown the server if they are
/// sent. When shuting down we save the data to disk.
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

    write_date_to_disk(&*data.read().await).expect("Failed to write data to disk");
}

use crate::{client::translate, ipc::Data};
use hbb_common::{allow_err, log, tokio};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn start_tray() {
    // Since we're not using the system tray icon, this function does not need to do anything.
}

pub fn make_tray() -> hbb_common::ResultType<()> {
    // This function was previously responsible for creating the system tray icon.
    // It has been left empty because the functionality has been removed.
    Ok(())
}

#[cfg(windows)]
#[tokio::main(flavor = "current_thread")]
async fn start_query_session_count(sender: std::sync::mpsc::Sender<Data>) {
    let mut last_count = 0;
    loop {
        if let Ok(mut c) = crate::ipc::connect(1000, "").await {
            let mut timer = tokio::time::interval(Duration::from_secs(1));
            loop {
                tokio::select! {
                    res = c.next() => {
                        match res {
                            Err(err) => {
                                log::error!("ipc connection closed: {}", err);
                                break;
                            }

                            Ok(Some(Data::ControlledSessionCount(count))) => {
                                if count != last_count {
                                    last_count = count;
                                    sender.send(Data::ControlledSessionCount(count)).ok();
                                }
                            }
                            _ => {}
                        }
                    }

                    _ = timer.tick() => {
                        c.send(&Data::ControlledSessionCount(0)).await.ok();
                    }
                }
            }
        }
        hbb_common::sleep(1.).await;
    }
}

// The rest of the code remains unchanged.
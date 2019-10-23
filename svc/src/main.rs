#[macro_use] extern crate windows_service;

use dotenv::dotenv;
use env_logger;
use std::ffi::OsString;
use std::time::Duration;
use windows_service::service_dispatcher;
use windows_service::service::{
    /*ServiceControl,*/ ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
    ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};

static SERVICE_NAME: &str = "dial-auto-svc";
define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(arguments: Vec<OsString>) {
    if let Err(_e) = run_service(arguments) {
        // Handle errors in some way.
    }
}

fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
//            ServiceControl::Stop | ServiceControl::Interrogate => {
//                ServiceControlHandlerResult::NoError
//            }

            evt => {
                log::info!("{:?}", evt);
                ServiceControlHandlerResult::NoError
            }
        }
    };

    // Register system service event handler
    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    let next_status = ServiceStatus {
        // Should match the one from system service registry
        service_type: ServiceType::OWN_PROCESS,
        // The new state
        current_state: ServiceState::Running,
        // Accept stop events when running
        controls_accepted: ServiceControlAccept::STOP,
        // Used to report an error when starting or stopping only, otherwise must be zero
        exit_code: ServiceExitCode::Win32(0),
        // Only used for pending states, otherwise must be zero
        checkpoint: 0,
        // Only used for pending states, otherwise must be zero
        wait_hint: Duration::default(),
    };

    // Tell the system that the service is running now
    status_handle.set_service_status(next_status)?;

    // Do some work

    Ok(())
}

#[cfg(windows)]
fn main() -> Result<(), windows_service::Error> {
    dotenv().ok();
    env_logger::builder()
        .write
    env_logger::init();
    log::info!("started");
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
    Ok(())
}

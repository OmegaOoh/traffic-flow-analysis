mod test_count;
mod test_flow;
mod test_desc;

use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio;
use dotenv::dotenv;
use std::env::var as env_var;
use reqwest;

struct ServerGuard(Option<Child>);

impl Drop for ServerGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            println!("Stopping backend server...");
            match child.kill() {
                Ok(_) => {
                    match child.wait() {
                        Ok(status) => println!("Backend server stopped with status: {:?}", status),
                        Err(e) => eprintln!("Error waiting for backend server to stop: {}", e),
                    }
                }
                Err(e) => eprintln!("Error killing backend server: {}", e),
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Backend Server and API Test");
    dotenv().ok();
    
    println!("Testing backend server...");
    let mut backend_test_cmd = match Command::new("cargo")
        .current_dir("../backend")
        .arg("test")
        .arg("-q")
        .stderr(Stdio::inherit())
        .spawn() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Failed to start server: {}", e),
    };
    let backend_test_cmd = backend_test_cmd.wait().unwrap();
    if !backend_test_cmd.success() {
        panic!("Backend tests failed");
    }

    println!("Starting backend server...");
    let cmd = match Command::new("cargo")
        .current_dir("../backend")
        .arg("run")
        .arg("-q")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Failed to start server: {}", e),
    };

    let check_url: String = env_var("ENDPOINT").expect("ENDPOINT not found");
    let check_url = check_url + "/alive";
    let client = reqwest::Client::new();
    let mut attempts = 0;
    const MAX_ATTEMPTS: i32 = 10;
    const POLL_INTERVAL: Duration = Duration::from_millis(2500);

    let _server_guard = 'waiting: loop {
        match client.get(&check_url).send().await {
            Ok(_) => {
                println!("Backend server is ready!");
                break 'waiting ServerGuard(Some(cmd));
            }
            Err(_) => {
                std::thread::sleep(POLL_INTERVAL);
                attempts += 1;
                println!("Attempt {} to connect to backend...", attempts);
                if attempts >= MAX_ATTEMPTS {
                    panic!("Failed to connect to server after {} attempts", MAX_ATTEMPTS);
                }
            }
        }
    };

    println!("Starting API Testing...");
    let mut test_cmd = match Command::new("cargo")
        .arg("test")
        .arg("-q")
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(err) => panic!("Failed to spawn test command: {}", err),
    };
    let test_status = test_cmd.wait().unwrap();
    if !test_status.success() {
        panic!("API tests failed");
    }
    
    // Frontend Testing
    println!("Starting Frontend Testing...");

    
    let mut frontend_install_cmd = match Command::new("bun")
        .current_dir("../frontend")
        .arg("install")
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(err) => panic!("Failed to spawn frontend install command: {}", err),
    };
    
    let frontend_install_status = frontend_install_cmd.wait().unwrap();
    if !frontend_install_status.success() {
        panic!("Frontend installation failed");
    }
    
    let mut frontend_prep_cmd = match Command::new("bun")
        .current_dir("../frontend")
        .arg("prepare")
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(err) => panic!("Failed to spawn frontend install command: {}", err),
    };
    
    let frontend_prep_status = frontend_prep_cmd.wait().unwrap();
    if !frontend_prep_status.success() {
        panic!("Frontend test prepare failed");
    }
    
    let mut frontend_test_cmd = match Command::new("bun")
        .current_dir("../frontend")
        .args(vec!["test"])
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(err) => panic!("Failed to spawn frontend test command: {}", err),
    };
    let frontend_test_status = frontend_test_cmd.wait().unwrap();
    if !frontend_test_status.success() {
        panic!("Frontend tests failed");
    }
    
    let mut frontend_e2e_cmd = match Command::new("bun")
        .current_dir("../frontend")
        .args(vec!["test:e2e:dev-test"])
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(err) => panic!("Failed to spawn frontend E2E test command: {}", err),
    };
    let frontend_e2e_status = frontend_e2e_cmd.wait().unwrap();
    if !frontend_e2e_status.success() {
        panic!("Frontend E2E tests failed");
    }
    
    println!("Exiting Process...");

    Ok(())
}
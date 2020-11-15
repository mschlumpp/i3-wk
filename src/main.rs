use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Output {
    name: String,
    active: bool,
    primary: bool,
    rect: Rect,
    current_workspace: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Workspace {
    num: u32,
    name: String,
    visible: bool,
    focused: bool,
    rect: Rect,
    output: String,
    urgent: bool,
}

fn read_outputs() -> Result<Vec<Output>, Box<dyn Error>> {
    let i3_out = Command::new("i3-msg")
        .arg("-t")
        .arg("get_outputs")
        .output()?;

    serde_json::de::from_slice(&i3_out.stdout).map_err(From::from)
}

fn read_workspaces() -> Result<Vec<Workspace>, Box<dyn Error>> {
    let i3_out = Command::new("i3-msg")
        .arg("-t")
        .arg("get_workspaces")
        .output()?;
    serde_json::de::from_slice(&i3_out.stdout).map_err(From::from)
}

fn to_workspace(wk: &str) -> Result<(), Box<dyn Error>> {
    Command::new("i3-msg")
        .arg(format!("workspace --no-auto-back-and-forth {}", wk))
        .status()
        .map_err(From::from)
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(From::from("Non-zero exit code"))
            }
        })
}

fn move_to_output(output: &str) -> Result<(), Box<dyn Error>> {
    Command::new("i3-msg")
        .arg(format!("move workspace to output {}", output))
        .status()
        .map_err(From::from)
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(From::from("Non-zero exit code"))
            }
        })
}

fn main() {
    // Parse arguments
    let target_name = match std::env::args().nth(1) {
        Some(t) => t,
        None => {
            println!("No target workspace provided\nUsage: i3-wk [workspace]");
            std::process::exit(1);
        }
    };

    let i3_outputs = read_outputs().expect("Cannot read outputs");
    let i3_workspaces = read_workspaces().expect("Cannot read workspaces");
    let focused_workspace = i3_workspaces
        .iter()
        .find(|x| x.focused)
        .expect("There must be at least one active output");
    let active_output = &focused_workspace.output;
    println!("Active output: {:#?}", active_output);

    if focused_workspace.name == target_name {
        println!("Workspace is already active...");
        return;
    }

    if let Some(target_workspace) = i3_workspaces.iter().find(|w| {
        // Find existing workspace on different output
        w.name == target_name && &w.output != active_output
    }) {
        println!("Workspace exists on other output");
        if target_workspace.visible {
            println!("Workspace is visible -> Exchange");

            let other_output = i3_outputs
                .iter()
                .find(|o| Some(&target_workspace.name) == o.current_workspace.as_ref())
                .expect("Cannot find output of target workspace");

            move_to_output(&other_output.name).expect("Cannot swap workspaces");
            to_workspace(&target_workspace.name).expect("Cannot focus target workspace");
            move_to_output(&active_output).expect("Cannot swap workspaces");
            to_workspace(&target_workspace.name).expect("Cannot focus moved workspace");
        } else {
            println!("Workspace is not visible -> Move");
            to_workspace(&target_name).expect("Couldn't switch to target workspace");
            move_to_output(&active_output).expect("Could not move workspace");
            to_workspace(&target_name).expect("Couldn't switch to target workspace");
        }
    } else {
        println!("Workspace doesn't exist or is on the same output -> Switch");
        to_workspace(&target_name).expect("Could not switch to workspace");
    }
}

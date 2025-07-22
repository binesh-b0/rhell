use std::fs;

// Removed the import of 'Ok' from anyhow to avoid conflict with pattern matching.

use crate::errors::CrateResult;
use std::path::Path;

pub fn pwd() -> CrateResult<String> {
    let current_dir = std::env::current_dir()?;
    Ok(current_dir.display().to_string())
}

// displays the prompt colorfully, curent folder  gitbranch and current user
pub fn prompt() -> CrateResult<String> {
    fn find_git_branch() -> Option<String> {
        let mut dir = std::env::current_dir().ok()?;
        loop {
            let git_head = dir.join(".git").join("HEAD");
            if git_head.exists() {
                if let Ok(head_contents) = std::fs::read_to_string(git_head) {
                    if head_contents.starts_with("ref:") {
                        let parts: Vec<&str> = head_contents.split('/').collect();
                        if let Some(branch) = parts.last() {
                            return Some(branch.trim().to_string());
                        }
                    }
                }
                break;
            }
            if let Some(parent) = dir.parent() {
                dir = parent.to_path_buf();
            } else {
                break;
            }
        }
        None
    }

    let user = std::env::var("USER").unwrap_or_else(|_| "user".into());
    let current_dir = std::env::current_dir()?
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("")
        .to_string();
    let branch = find_git_branch().unwrap_or_else(|| "".to_string());

    // ANSI escape codes: green for user, blue for folder, magenta for git branch
    let prompt = format!(
        "\n\x1b[32m{}\x1b[0m in \x1b[34m{}\x1b[0m \x1b[35m\u{e0a0} {}\x1b[0m",
        user, current_dir, branch
    );
    Ok(prompt)
}


/// Command Helpers

pub fn ls() -> CrateResult<()> {  
    let entries = fs::read_dir(".")?;   

    for entry in entries {      
        let entry = entry?;      
         println!("{}", entry.file_name().to_string_lossy());  
    }   

    Ok(()) 
}

pub fn cd(path: &str) -> CrateResult<()> {
    std::env::set_current_dir(path)?;
    Ok(())
}

pub fn touch(path: &str) -> CrateResult<()> {
    fs::File::create(path)?;
    Ok(())
}

pub fn rm(path: &str) -> CrateResult<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn cat(path: &str) -> CrateResult<String> {
    let pwd = pwd()?;
    let joined_path = std::path::Path::new(&pwd).join(path);
    let contents = fs::read_to_string(joined_path)?;
    Ok(contents)
}
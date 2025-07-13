use std::process::Command;

fn is_git_installed() -> bool {
    match Command::new("git").arg("--version").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

pub fn git_root_project() -> Option<String> {
    if !is_git_installed() {
        return None;
    }
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                let root = String::
                    from_utf8_lossy(&output.stdout).trim().to_string();
                Some(root)
            } else {
                None
            }
        },
        Err(_) => None,
    }
}

pub fn is_git_repository() -> bool {
    return git_root_project().is_some();
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_git_installed() {
        assert!(is_git_installed(), "Git should be installed on the system.");
    }

    #[test]
    fn test_git_root_project() {
        if let Some(root) = git_root_project() {
            assert!(!root.is_empty(), "Git root project should not be empty.");
            let output = Command::new("ls")
                .arg("-a")
                .arg(&root)
                .output()
                .expect("Failed to list files in git root project.");
            let files = String::from_utf8_lossy(&output.stdout);
            assert!(files.contains(".git"), "Git root project should contain a .git directory.");
        } else {
            assert!(false, "Git root project should be determined if in a git repository.");
        }
    }
}

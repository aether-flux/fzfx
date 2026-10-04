/// Extract command name from raw command
fn extract_base_binary(mut cmd: &str) -> &str {
    cmd = cmd.trim();
    if cmd.starts_with("sudo") {
        cmd = cmd.split("sudo").collect::<Vec<&str>>()[1];
    }
    cmd.split_whitespace().next().unwrap_or("")
}

/// Get command description
pub fn get_cmd_desc(cmd: &str) -> Option<&'static str> {
    let base = extract_base_binary(cmd);

    match base {
        "rm" => Some("remove delete files directories unlink erase"),
        "cp" => Some("copy duplicate replicate files directories"),
        "mv" => Some("move rename relocate files directories"),
        "ls" => Some("list directory contents files folder display"),
        "cat" => Some("print view display concatenated file contents text"),
        "grep" | "ripgrep" | "rg" => Some("search pattern text match find string inside file"),
        "df" => Some("disk free space usage filesystem available storage capacity"),
        "du" => Some("disk usage file space size directory capacity"),
        "fdisk" => Some("manipulate disk partition table"),
        "ps" => Some("process status active running tasks programs"),
        "kill" | "pkill" | "killall" => Some("terminate stop end process signal force quit"),
        "find" => Some("locate search find files directories paths name pattern"),
        "chmod" => Some("change permissions mode read write execute access"),
        "chown" => Some("change owner user group permissions access"),
        "ln" => Some("link symlink symbolic hard link shortcut target"),
        "lsof" => Some("list open files network ports sockets listening processes"),
        "netstat" | "ss" => Some("network sockets listening ports connections status statistics"),
        "curl" | "wget" => Some("http request fetch download web page endpoint api URL"),
        "ping" => Some("network connectivity latency host check reachable ICMP"),
        "docker" => Some("container image process pod volume swarm engine"),
        "kubectl" | "k8s" => {
            Some("kubernetes cluster pod deployment service container orchestration")
        }
        "git" => Some("version control repository commit branch merge checkout push pull diff"),

        _ => None,
    }
}

/// Convert raw command into command with context
pub fn add_cmd_context(cmd: &str) -> String {
    if let Some(desc) = get_cmd_desc(cmd) {
        format!("{} ({})", cmd, desc)
    } else {
        cmd.to_string()
    }
}

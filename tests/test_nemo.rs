use std::process::Command;

#[test]
pub fn test_nemo_new() {
    let project_name = "nemo_test";

    let command = Command::new("nemo")
        .arg("new")
        .arg("--name")
        .arg(project_name)
        .status()
        .expect("failed to execute process");
    assert_eq!(command.success(), true);
    
    let project_path = format!("./{}", project_name);
    let project_exists = std::path::Path::new(&project_path).exists();
    assert_eq!(project_exists, true);

    std::fs::remove_dir_all(&project_path).expect("failed to remove directory");
}

#[test]
pub fn test_nemo_init() {
    let project_name = "nemo_test_1233245435234:";

    let command = Command::new("nemo")
        .current_dir(format!("./{}", project_name))
        .arg("init")
        .status()
        .expect("failed to execute process");
    assert_eq!(command.success(), true);

    let project_path = format!("./{}", project_name);
    let project_exists = std::path::Path::new(&project_path).exists();
    assert_eq!(project_exists, true);

    std::fs::remove_dir_all(&project_path).expect("failed to remove directory");
}

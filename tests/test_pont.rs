use std::process::Command;

#[test]
pub fn test_pont_new() {
    let project_name = "pont_test_new";

    let command = Command::new("pont")
        .arg("new")
        .arg("--name")
        .arg(project_name)
        .status()
        .expect("failed to create new pont project");
    assert_eq!(command.success(), true);

    let project_path = format!("./{}", project_name);
    let project_exists = std::path::Path::new(&project_path).exists();
    assert_eq!(project_exists, true);

    std::fs::remove_dir_all(&project_path).expect("failed to remove directory");
}

#[test]
pub fn test_pont_init() {
    let project_name = "pont_test_init";
    let project_path = format!("./{}", project_name);

    std::fs::create_dir(project_name).expect("failed to create directory");

    let command = Command::new("pont")
        .current_dir(&project_path)
        .arg("init")
        .status()
        .expect("failed to init pont project");
    assert_eq!(command.success(), true);

    let project_exists = std::path::Path::new(&project_path).exists();
    assert_eq!(project_exists, true);

    std::fs::remove_dir_all(&project_path).expect("failed to remove directory");
}

#[test]
pub fn test_pont_build() {
    let template_name = "pont_test_template";
    let template_path = format!("./{}", template_name);

    let new_command = Command::new("pont")
        .arg("new")
        .arg("--name")
        .arg(template_name)
        .status()
        .expect("failed to create new pont project");
    assert_eq!(new_command.success(), true);

    std::fs::File::create(format!("{}/test.txt", template_path)).expect("failed to create file");

    let project_name = "pont_test_build";
    let command = Command::new("pont")
        .arg("build")
        .arg("--name")
        .arg(project_name)
        .arg("--from")
        .arg(format!("file://{}", template_name))
        .status()
        .expect("failed to build pont project");
    assert_eq!(command.success(), true);

    let expected_project_path = format!("./{}", project_name);
    let project_exists = std::path::Path::new(&expected_project_path).exists();
    assert_eq!(project_exists, true);

    let expected_file_path = format!("{}/test.txt", expected_project_path);
    let file_exists = std::path::Path::new(&expected_file_path).exists();
    assert_eq!(file_exists, true);

    std::fs::remove_dir_all(&project_name).expect("failed to remove directory");
    std::fs::remove_dir_all(&template_path).expect("failed to remove directory");
}

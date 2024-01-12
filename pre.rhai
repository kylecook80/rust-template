let config_file_name = variable::get("project-name");
append(config_file_name, ".toml");
file::rename("app.toml", config_file_name);

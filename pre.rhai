let config_file_name = variable::get("crate_name");
append(config_file_name, ".toml");
file::rename("app.toml", config_file_name);

let config_enable = variable::get("config_enable");
if config_enable == false {
    file::delete("src/config.rs");
    file::delete(config_file_name);
}
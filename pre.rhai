let config_enable = variable::get("config_enable");
if config_enable == false {
    file::delete("src/config.rs");
}
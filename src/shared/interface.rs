pub trait PlatformPrinterGetters {

    fn get_name(&self) -> String;

    fn get_is_default(&self) -> bool;

    fn get_system_name(&self) -> String;

    fn get_marker_and_model(&self) -> String;

    fn get_is_shared(&self) -> bool;

    fn get_uri(&self) -> String;

    fn get_location(&self) -> String;

    fn get_state(&self) -> String;

}
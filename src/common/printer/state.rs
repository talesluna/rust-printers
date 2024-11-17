/**
 * Enum of the Printer state
 */
#[derive(Debug, Clone)]
pub enum PrinterState {

    /**
     * The printer is able to receive jobs (also idle)
     */
    READY,

    /**
     * The printer is not accepting jobs (also stopped)
     */
    PAUSED,

    /**
     * The printer is now printing an document (also processing)
     */
    PRINTING,

    /**
     * All other status like error, resources, manual intervention, etc...
     */
    UNKNOWN,

}

impl PrinterState {

    #[cfg(target_family = "unix")]
    pub fn from_platform_state(platform_state: &str) -> Self {
        if platform_state == "3" {
            return PrinterState::READY;
        }
        
        if platform_state == "4" {
            return PrinterState::PRINTING;
        }

        if platform_state == "5" {
            return PrinterState::PAUSED;
        }

        return PrinterState::UNKNOWN;
    }

    #[cfg(target_family = "windows")]
    pub fn from_platform_state(platform_state: &str) -> Self {

        if platform_state == "0" {
            return PrinterState::READY;
        }

        if platform_state == "1" || platform_state == "2" {
            return PrinterState::PAUSED;
        }

        if platform_state == "5" {
            return PrinterState::PRINTING;
        }

        return PrinterState::UNKNOWN;

    }
}
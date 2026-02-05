use std::backtrace::Backtrace;

/**
 * Known structured errors 
 */
#[derive(Debug)]
pub struct PrintersError {
    /**
     * Specific error message
     */
    pub message: String,
    /**
     * Generic failure idenfier
     */
    pub failure: PrintersFailure,
    /**
     * Available backtrace
     */
    pub backtrace: Backtrace,
}

#[derive(Debug, PartialEq)]
pub enum PrintersFailure {
    FileFailure,
    PrintFailure,
    GenericFailure,
    ConverterFailure,
    JobFailure,
}

impl PrintersError {
    fn new<E>(error: E, failure: PrintersFailure) -> Self
    where
        E: std::fmt::Display,
    {
        Self {
            failure,
            message: error.to_string(),
            backtrace: Backtrace::capture(),
        }
    }

    pub fn file_error<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, PrintersFailure::FileFailure)
    }

    pub fn converter_error<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, PrintersFailure::ConverterFailure)
    }

    pub fn print_error<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, PrintersFailure::PrintFailure)
    }

    pub fn job_error<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, PrintersFailure::JobFailure)
    }

    pub fn error<E>(error: E) -> Self
    where
        E: std::fmt::Display,
    {
        Self::new(error, PrintersFailure::GenericFailure)
    }
}

use crate::rendering::PreparedFlowOutput;
use std::process::ExitCode;

pub(crate) fn success() -> ExitCode {
    ExitDisposition::Success.exit_code()
}

pub(crate) fn failure() -> ExitCode {
    ExitDisposition::Failure.exit_code()
}

pub(crate) fn flow_output(output: &PreparedFlowOutput) -> ExitCode {
    if output.is_ready() {
        success()
    } else {
        failure()
    }
}

pub(crate) fn doctor_report(report: &handbook_compiler::DoctorReport) -> ExitCode {
    if report.status == handbook_compiler::DoctorBaselineStatus::BaselineComplete {
        success()
    } else {
        failure()
    }
}

enum ExitDisposition {
    Success,
    Failure,
}

impl ExitDisposition {
    fn exit_code(self) -> ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::Failure => ExitCode::from(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_and_failure_match_shell_contract() {
        assert_eq!(success(), ExitCode::SUCCESS);
        assert_eq!(failure(), ExitCode::from(1));
    }
}

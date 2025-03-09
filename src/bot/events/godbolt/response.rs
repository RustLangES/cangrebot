use serde::Deserialize;

macro_rules! aggregate {
    ($input:expr) => {{
        $input
            .into_iter()
            .map(|entry| entry.text)
            .collect::<Vec<_>>()
            .join("\n")
    }}
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CompilerResponse {
    AssemblyResponse(BaseAssemblyResponse),
    RunResponse(BaseRunResponse)
}

#[derive(Deserialize)]
pub struct BaseAssemblyResponse {
    asm: Vec<DataEntry>,
    stdout: Vec<DataEntry>,
    stderr: Vec<DataEntry>,
    #[serde(rename(deserialize = "code"))]
    exit_code: i32
}

#[derive(Deserialize)]
pub struct BaseRunResponse {
    stdout: Vec<DataEntry>,
    stderr: Vec<DataEntry>,
    #[serde(rename(deserialize = "didExecute"))]
    did_run: bool,
    #[serde(rename(deserialize = "buildResult"))]
    build_result: RunCompilerOutput
}

#[derive(Deserialize)]
pub struct RunCompilerOutput {
    stdout: Vec<DataEntry>,
    stderr: Vec<DataEntry>,
}

#[derive(Deserialize)]
pub struct DataEntry {
    text: String
}

impl CompilerResponse {
    pub fn aggregate_run_out(self) -> String {
        match self {
            Self::AssemblyResponse(res) => res.aggregate_source(),
            Self::RunResponse(res) => res.aggregate_run_out()
        }
    }

    pub fn aggregate_comp_out(self) -> String {
        match self {
            Self::AssemblyResponse(res) => res.aggregate_comp_out(),
            Self::RunResponse(res) => res.aggregate_comp_out()
        }
    }

    pub fn is_success(&self) -> bool {
        match self {
            Self::AssemblyResponse(res) => res.success(),
            Self::RunResponse(res) => res.success()
        }
    }
}

impl BaseAssemblyResponse {
    pub fn aggregate_source(self) -> String {
        aggregate!(self.asm)
    }

    pub fn aggregate_comp_out(self) -> String {
        let mut aggregated = Vec::new();

        aggregated.extend(self.stderr);
        aggregated.extend(self.stdout);

        aggregate!(aggregated)
    }

    pub fn success(&self) -> bool {
        self.exit_code == 0
    }
}

impl BaseRunResponse {
    pub fn aggregate_run_out(self) -> String {
        let mut aggregated = Vec::new();

        aggregated.extend(self.stderr);
        aggregated.extend(self.stdout);

        aggregate!(aggregated)
    }

    pub fn aggregate_comp_out(self) -> String {
        let mut aggregated = Vec::new();

        aggregated.extend(self.build_result.stderr);
        aggregated.extend(self.build_result.stdout);

        aggregate!(aggregated)
    }

    pub fn success(&self) -> bool {
        self.did_run
    }
}

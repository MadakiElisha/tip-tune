use soroban_sdk::{Env, BytesN};

pub struct GasProfile {
    pub cpu_insns: u64,
    pub mem_bytes: u64,
}

impl GasProfile {
    pub fn new(env: &Env) -> Self {
        let cpu_insns = env.budget().cpu_insns();
        let mem_bytes = env.budget().mem_bytes();
        Self { cpu_insns, mem_bytes }
    }

    pub fn delta(&self, after: &GasProfile) -> GasProfile {
        GasProfile {
            cpu_insns: after.cpu_insns - self.cpu_insns,
            mem_bytes: after.mem_bytes - self.mem_bytes,
        }
    }
}
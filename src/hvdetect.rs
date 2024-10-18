#[cfg(target_arch = "x86_64")]
use raw_cpuid::CpuId;

use anyhow::Result;
use crate::{Test, TestResult};

pub struct HypervisorTest {}

#[derive(Default)]
pub struct HypervisorResult {
    pub hypervisor: String,
    pub present: bool,
}

#[cfg(target_arch = "x86_64")]
fn xen_probe() -> Result<bool, ()> {
    let cpuid = CpuId::new();

    let has_hypervisor = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_hypervisor());
    if !has_hypervisor {
        return Ok(false);
    }

    let has_xen = cpuid.get_hypervisor_info().map_or(false, |hinfo| hinfo.identify() == raw_cpuid::Hypervisor::Xen);
    if !has_xen {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(not(target_arch = "x86_64"))]
fn xen_probe() -> Result<bool, ()> {
    Ok(false)
}

#[cfg(target_arch = "x86_64")]
fn kvm_probe() -> Result<bool, ()> {
    let cpuid = CpuId::new();

    let has_hypervisor = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_hypervisor());
    if !has_hypervisor {
        return Ok(false);
    }

    let has_kvm = cpuid.get_hypervisor_info().map_or(false, |hinfo| hinfo.identify() == raw_cpuid::Hypervisor::KVM);
    if !has_kvm {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(not(target_arch = "x86_64"))]
fn kvm_probe() -> Result<bool, ()> {
    Ok(false)
}

#[cfg(target_arch = "x86_64")]
fn bhyve_probe() -> Result<bool, ()> {
    let cpuid = CpuId::new();

    let has_hypervisor = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_hypervisor());
    if !has_hypervisor {
        return Ok(false);
    }

    let has_bhyve = cpuid.get_hypervisor_info().map_or(false, |hinfo| hinfo.identify() == raw_cpuid::Hypervisor::Bhyve);
    if !has_bhyve {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(not(target_arch = "x86_64"))]
fn bhyve_probe() -> Result<bool, ()> {
    Ok(false)
}

#[cfg(target_arch = "x86_64")]
fn hyperv_probe() -> Result<bool, ()> {
    let cpuid = CpuId::new();

    let has_hypervisor = cpuid.get_feature_info().map_or(false, |finfo| finfo.has_hypervisor());
    if !has_hypervisor {
        return Ok(false);
    }

    let has_hyperv = cpuid.get_hypervisor_info().map_or(false, |hinfo| hinfo.identify() == raw_cpuid::Hypervisor::HyperV);
    if !has_hyperv {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(not(target_arch = "x86_64"))]
fn hyperv_probe() -> Result<bool, ()> {
    Ok(false)
}

impl Test for HypervisorTest {
    fn name(&self) -> String {
        "whether we are running under a hypervisor".to_string()
    }

    fn run(&self) -> Result<Box<dyn TestResult>, ()> {
        let mut result = HypervisorResult{
            hypervisor: "unknown".to_string(),
            present: false,
        };

        if xen_probe()? {
            result.hypervisor = "Xen".to_string();
            result.present = true;
        }

        if kvm_probe()? {
            result.hypervisor = "KVM".to_string();
            result.present = true;
        }

        if bhyve_probe()? {
            result.hypervisor = "bhyve".to_string();
            result.present = true;
        }

        if hyperv_probe()? {
            result.hypervisor = "Hyper-V".to_string();
            result.present = true;
        }

        Ok(Box::new(result))
    }
}

impl TestResult for HypervisorResult {
    fn success(&self) -> bool {
        self.present
    }

    fn explain(&self) {
        if !self.present {
            println!("  - No hypervisor detected.");
            println!("  - Why: VM-based barriers provide the strongest isolation guarantees");
            println!("         for workloads.");
            println!("  - Suggestion: Use a hypervisor-based solution such as Edera Protect,");
            println!("                Kata Containers or Firecracker.");
            return;
        }

        println!("  + {} hypervisor control surface detected.", self.hypervisor);
    }

    fn as_string(&self) -> String {
        if self.present {
            return "present".to_string();
        }

        "not present".to_string()
    }
}

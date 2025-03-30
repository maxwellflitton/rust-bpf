use anyhow::{anyhow, Context as _};
use aya_build::cargo_metadata;


fn build(package_name: &str) -> anyhow::Result<()> {
    let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()
        .context("MetadataCommand::exec")?;
    let ebpf_package = packages
        .into_iter()
        .find(|cargo_metadata::Package { name, .. }| name == package_name)
        .ok_or_else(|| anyhow!("{} package not found", package_name))?;
    aya_build::build_ebpf([ebpf_package])
}

fn main() -> anyhow::Result<()> {
    let _ = build("ingress-ebpf");
    let _ = build("egress-ebpf");
    Ok(())

}

mod client;
mod publish;
mod pull;
mod reference;

pub(super) use publish::remote_publish;
pub(super) use pull::remote_pull;
pub(super) use reference::is_remote_ref;

const OCI_CONFIG_MEDIA_TYPE: &str = "application/vnd.underlay.migration.bundle.config.v1+json";
const OCI_PACKAGE_LAYER_MEDIA_TYPE: &str = "application/vnd.underlay.bundle.package.v1+json";

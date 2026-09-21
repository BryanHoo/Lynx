//! Provides constructs for the Lynx app version and release channel.

#![deny(missing_docs)]

use std::{env, sync::LazyLock};

use gpui::{App, Global};
use semver::Version;

const LYNX_DOCS_DIRECTORY_URL: &str = "https://github.com/BryanHoo/Lynx/tree/main/docs/src";
const LYNX_DOCS_FILE_URL: &str = "https://github.com/BryanHoo/Lynx/blob/main/docs/src";

/// The fixed release channel name used for data-directory isolation.
pub static RELEASE_CHANNEL_NAME: LazyLock<String> = LazyLock::new(|| "lynx".to_owned());

#[doc(hidden)]
pub static RELEASE_CHANNEL: LazyLock<ReleaseChannel> = LazyLock::new(ReleaseChannel::default);

/// The app identifier for the current release channel, Windows only.
#[cfg(target_os = "windows")]
pub fn app_identifier() -> &'static str {
    "Lynx"
}

/// The Git commit SHA that Lynx was built at.
#[derive(Clone, Eq, Debug, PartialEq)]
pub struct AppCommitSha(String);

struct GlobalAppCommitSha(AppCommitSha);

impl Global for GlobalAppCommitSha {}

impl AppCommitSha {
    /// Creates a new [`AppCommitSha`].
    pub fn new(sha: String) -> Self {
        AppCommitSha(sha)
    }

    /// Returns the global [`AppCommitSha`], if one is set.
    pub fn try_global(cx: &App) -> Option<AppCommitSha> {
        cx.try_global::<GlobalAppCommitSha>()
            .map(|sha| sha.0.clone())
    }

    /// Sets the global [`AppCommitSha`].
    pub fn set_global(sha: AppCommitSha, cx: &mut App) {
        cx.set_global(GlobalAppCommitSha(sha))
    }

    /// Returns the full commit SHA.
    pub fn full(&self) -> String {
        self.0.to_string()
    }

    /// Returns the short (7 character) commit SHA.
    pub fn short(&self) -> String {
        self.0.chars().take(7).collect()
    }
}

struct GlobalAppVersion(Version);

impl Global for GlobalAppVersion {}

/// The version of Lynx.
pub struct AppVersion;

impl AppVersion {
    /// Load the app version from env.
    pub fn load(
        pkg_version: &str,
        build_id: Option<&str>,
        commit_sha: Option<AppCommitSha>,
    ) -> Version {
        let mut version: Version = if let Ok(from_env) = env::var("ZED_APP_VERSION") {
            from_env.parse().expect("invalid ZED_APP_VERSION")
        } else {
            pkg_version.parse().expect("invalid version in Cargo.toml")
        };
        let mut pre = String::from(RELEASE_CHANNEL.dev_name());

        if let Some(build_id) = build_id {
            pre.push('.');
            pre.push_str(&build_id);
        }

        if let Some(sha) = commit_sha {
            pre.push('.');
            pre.push_str(&sha.0);
        }
        if let Ok(build) = semver::BuildMetadata::new(&pre) {
            version.build = build;
        }

        version
    }

    /// Returns the global version number.
    pub fn global(cx: &App) -> Version {
        if cx.has_global::<GlobalAppVersion>() {
            cx.global::<GlobalAppVersion>().0.clone()
        } else {
            Version::new(0, 0, 0)
        }
    }
}

/// The Lynx release channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum ReleaseChannel {
    /// The only Lynx release channel.
    #[default]
    Lynx,
}

struct GlobalReleaseChannel(ReleaseChannel);

impl Global for GlobalReleaseChannel {}

/// Initializes the release channel.
pub fn init(app_version: Version, cx: &mut App) {
    cx.set_global(GlobalAppVersion(app_version));
    cx.set_global(GlobalReleaseChannel(*RELEASE_CHANNEL))
}

/// Initializes the app version and release channel for tests.
pub fn init_test(app_version: Version, release_channel: ReleaseChannel, cx: &mut App) {
    cx.set_global(GlobalAppVersion(app_version));
    cx.set_global(GlobalReleaseChannel(release_channel))
}

/// Returns the Lynx docs URL for the current release channel for the given
/// `slug`.
pub fn docs_url(slug: &str, cx: &App) -> String {
    ReleaseChannel::try_global(cx)
        .unwrap_or(*RELEASE_CHANNEL)
        .docs_url(slug)
}

impl ReleaseChannel {
    /// Returns the global [`ReleaseChannel`].
    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalReleaseChannel>().0
    }

    /// Returns the global [`ReleaseChannel`], if one is set.
    pub fn try_global(cx: &App) -> Option<Self> {
        cx.try_global::<GlobalReleaseChannel>()
            .map(|channel| channel.0)
    }

    /// Returns the display name for this [`ReleaseChannel`].
    pub fn display_name(&self) -> &'static str {
        "Lynx"
    }

    /// Returns the programmatic name for this [`ReleaseChannel`].
    pub fn dev_name(&self) -> &'static str {
        "lynx"
    }

    /// Returns the application ID that's used by Wayland as application ID
    /// and WM_CLASS on X11.
    /// This also has to match the bundle identifier for Lynx on macOS.
    pub fn app_id(&self) -> &'static str {
        "dev.lynx.Lynx"
    }

    /// Returns the Lynx docs URL for the given `slug`.
    pub fn docs_url(&self, slug: &str) -> String {
        if slug.is_empty() {
            LYNX_DOCS_DIRECTORY_URL.to_owned()
        } else {
            let (path, fragment) = slug
                .split_once('#')
                .map_or((slug, None), |(path, fragment)| (path, Some(fragment)));
            let mut url = format!("{LYNX_DOCS_FILE_URL}/{path}.md");
            if let Some(fragment) = fragment {
                url.push('#');
                url.push_str(fragment);
            }
            url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RELEASE_CHANNEL_NAME, ReleaseChannel};

    #[test]
    fn release_channel_uses_lynx_identity() {
        let channel = ReleaseChannel::default();

        assert_eq!(&*RELEASE_CHANNEL_NAME, "lynx");
        assert_eq!(channel.dev_name(), "lynx");
        assert_eq!(channel.display_name(), "Lynx");
        assert_eq!(channel.app_id(), "dev.lynx.Lynx");
    }

    #[test]
    fn test_docs_url_for_release_channel() {
        assert_eq!(
            ReleaseChannel::Lynx.docs_url("settings"),
            "https://github.com/BryanHoo/Lynx/blob/main/docs/src/settings.md"
        );
        assert_eq!(
            ReleaseChannel::Lynx.docs_url("tasks#custom-git-commands"),
            "https://github.com/BryanHoo/Lynx/blob/main/docs/src/tasks.md#custom-git-commands"
        );
    }
}

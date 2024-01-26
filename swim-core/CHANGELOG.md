# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.0 (2024-01-26)

<csr-id-e5818f52fd0b8f21b7a92747a951cd47362a0eb8/>
<csr-id-27c148e50661798b7373bd6947ea8c2eff0adee8/>

### Other

 - <csr-id-e5818f52fd0b8f21b7a92747a951cd47362a0eb8/> tokio to 1.35
 - <csr-id-27c148e50661798b7373bd6947ea8c2eff0adee8/> default request logging middleware

### Chore

 - <csr-id-e4669abeef822dea9151346daeadf1b91b3b8d40/> initialise CHANGELOG.md for each crate

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 31 commits contributed to the release over the course of 389 calendar days.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Initialise CHANGELOG.md for each crate ([`e4669ab`](https://github.com/SyedAhkam/swim/commit/e4669abeef822dea9151346daeadf1b91b3b8d40))
    - Tokio to 1.35 ([`e5818f5`](https://github.com/SyedAhkam/swim/commit/e5818f52fd0b8f21b7a92747a951cd47362a0eb8))
    - Add STUF warning for Model trait ([`8188b13`](https://github.com/SyedAhkam/swim/commit/8188b13dff511a7300a28b3d6b2916e8b73d12de))
    - Update docs for swim macro ([`7fecb15`](https://github.com/SyedAhkam/swim/commit/7fecb1505405a22e4f36849189213ef403423967))
    - Improve error handling ([`11915da`](https://github.com/SyedAhkam/swim/commit/11915da63f60e8ae9d92d20f7abfe0fd4dbc6c3b))
    - Improve error handling ([`58b6ebc`](https://github.com/SyedAhkam/swim/commit/58b6ebc6c1f69e87b0b7f426f2f6c31bb266e075))
    - Default request logging middleware ([`27c148e`](https://github.com/SyedAhkam/swim/commit/27c148e50661798b7373bd6947ea8c2eff0adee8))
    - Add some sweet project logs ([`7c2712b`](https://github.com/SyedAhkam/swim/commit/7c2712bbdbe7cb58d8a462e21298aafccd9d56ee))
    - Release 0.2.1 ([`c803d09`](https://github.com/SyedAhkam/swim/commit/c803d093573d2210a3812bc6958e5f5360ae613d))
    - Re-export internal crates ([`efd8d22`](https://github.com/SyedAhkam/swim/commit/efd8d22b403c8c2e10b22cd2c5ad4d93cb845e87))
    - Fix doc tests ([`e27f6c3`](https://github.com/SyedAhkam/swim/commit/e27f6c374bdc4da912065d4112371467725eaa71))
    - Release 0.2.0 ([`c47777b`](https://github.com/SyedAhkam/swim/commit/c47777bc32d57673378d3ad7db2c236fc8f060d6))
    - Add description field to cargo metadata ([`101472e`](https://github.com/SyedAhkam/swim/commit/101472eabdb5424e468bc1e9f29af1033612d411))
    - Revert "Release 0.2.0" yet again ([`85f503c`](https://github.com/SyedAhkam/swim/commit/85f503c97d89a3df101dc756949e020eb94585c6))
    - Release 0.2.0 ([`b42c941`](https://github.com/SyedAhkam/swim/commit/b42c9419a52b7f663c93e9d23809d2d1288a26f7))
    - Add cargo metadata ([`73bb332`](https://github.com/SyedAhkam/swim/commit/73bb332a1c63e7548dbb73b040994384ba532f5d))
    - Revert "Release 0.2.0" ([`c67754d`](https://github.com/SyedAhkam/swim/commit/c67754d402cd7bb894ba6c9e7c20000e9f7a140f))
    - Release 0.2.0 ([`d10fea2`](https://github.com/SyedAhkam/swim/commit/d10fea2f99a6d2e76a867697e64a86cd486a0b15))
    - Implement middlewares API ⚡ ([`26bced6`](https://github.com/SyedAhkam/swim/commit/26bced659ad9109dc4f0ef5f1c6c5b75005dd44f))
    - Return 404 when get method is not impl'd ([`b6fc83c`](https://github.com/SyedAhkam/swim/commit/b6fc83c8bcd0eda20fd34894120756b994b033a6))
    - Build a custom Error type ([`d39f74c`](https://github.com/SyedAhkam/swim/commit/d39f74c8b5a0f7501a2880086c23b659d7b9188c))
    - Implement app based routing ✨ ([`97db013`](https://github.com/SyedAhkam/swim/commit/97db0135013a169581e964ae39f79e181d9831ed))
    - Make App and Middleware thread safe ([`67fe271`](https://github.com/SyedAhkam/swim/commit/67fe2719976c2bbc235bbf8bee6768c5f46d25ba))
    - Implement `Route` and `View` API ([`f94e736`](https://github.com/SyedAhkam/swim/commit/f94e736f17637fbd186f3246817eceb6ff8fcbdc))
    - Implement the `App` module API ([`28ee3d7`](https://github.com/SyedAhkam/swim/commit/28ee3d7190e0d656744d50a23af44090409f93fb))
    - Use hyper internally to bind the server ([`369eefb`](https://github.com/SyedAhkam/swim/commit/369eefbab09cb5e923a246f64aef9df5d3c199cc))
    - Implement the Settings API ([`43dab1f`](https://github.com/SyedAhkam/swim/commit/43dab1fb5cfccc5c43a727e18caf2e001cfb1ba9))
    - Create blank settings builder and setup tokio ([`85f4993`](https://github.com/SyedAhkam/swim/commit/85f4993bb8ffaf4cefdeb081c8279b3249ed80f1))
    - Accept expressions instead of literals in swim macro ([`8bf2a03`](https://github.com/SyedAhkam/swim/commit/8bf2a03295f5f3b6ff52cdc6e82b992ec511f929))
    - Lots of progress! ([`f42ba5f`](https://github.com/SyedAhkam/swim/commit/f42ba5ff28cb7888408abb97f384f4e7e85e3182))
    - Set up a cargo workspace with dummy crates ([`8a2c625`](https://github.com/SyedAhkam/swim/commit/8a2c62550f6f26be4aac9872bef037d74947c834))
</details>


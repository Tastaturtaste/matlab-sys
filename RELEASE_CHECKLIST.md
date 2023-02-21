# Instructions 
1. Make a dedicated branch to prepare the release. All features, bugfixes, etc. intended for the release should already be in the `dev` branch. Some small fixes can be done in the release branch. As the branch should be named after the intended release version, check if a minor version bump is sufficient or a major version bump is required. Tools like [cargo-semver-checks](https://crates.io/crates/cargo-semver-checks) can be used for support.
   1. `git checkout dev`
   2. `cargo semver-checks check-release`
   3. `git checkout -b release-_version_`
2. Make sure everything works and all tests pass!
   1. `cargo xtask test`
3. Give the documentation and the README a last look-over. 
4. Bump the version number in `./matlab-sys/Cargo.toml` to the previously decided upon version.
5. Switch to the `master` branch for the release, merge the `release-_version_` branch and tag the commit on `master`.
   1. `git switch master`
   2. `git merge --no-ff release-_version_`
   3. `git tag -a _version_`
6. Push master.
   1. `git push origin master`
7. Publish the release to [crates.io](https://crates.io/) with the custom publish task.
   1. `cargo xtask publish`
8. Merge the changes made in the `release-_version` branch back into the `dev` branch.
   1. `git checkout dev`
   2. `git merge --no-ff release-_version_`
9.  Clean up the release branch, the next release will get a new branch.
   1.  `git branch -d release-_version_`


The workflow described above follows the advice on https://nvie.com/posts/a-successful-git-branching-model/
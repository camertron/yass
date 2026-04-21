# Releasing Yass

Follow these steps to release a new version of Yass:

1. Bump the version in lib/yass/version.rb.
2. Bump the version in ext/yass/Cargo.toml (should be the same version from step 1).
3. Add a heading in CHANGELOG.md with the version number and today's date.
4. Run `bundle install` and `bundle exec rake compile` to refresh the Bundler and Cargo lockfiles.
5. Commit the results of these changes and push them to GitHub.
6. Wait for CI to run. This might seem unimportant but weird things can go wrong with lockfiles, etc, so it's best to wait for a clean bill of health before continuing.
7. Create a release in the GitHub UI [here](https://github.com/camertron/yass/releases/new).
    1. Create a new tag for the release of the form vX.X.X, using the same version number you've been using this whole time.
    2. Name the release the same thing, i.e. vX.X.X.
    3. Copy the relevant section from CHANGELOG.md (including the header) and paste it into the release notes.
8. This will kick off the GitHub action in .github/workflows/release.yml, build the native extension for all the supported ruby versions and platforms, and publish the various .gem files to rubygems.org.

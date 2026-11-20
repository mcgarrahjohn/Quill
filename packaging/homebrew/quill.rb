# Homebrew cask for Quill. Put this file in a GitHub repo named `homebrew-quill`
# at `Casks/quill.rb`; then users can run:
#   brew install --cask mcgarrahjohn/quill/quill
# Run `packaging/homebrew/update-cask.sh <version>` after each release to fill in the version and checksums.
cask "quill" do
  arch arm: "aarch64", intel: "x64"

  version "0.1.0"
  sha256 arm:   "REPLACE_WITH_AARCH64_DMG_SHA256",
         intel: "REPLACE_WITH_X64_DMG_SHA256"

  url "https://github.com/mcgarrahjohn/quill/releases/download/v#{version}/Quill_#{version}_#{arch}.dmg"
  name "Quill"
  desc "Private, on-device voice dictation: speak and your words appear in any app"
  homepage "https://github.com/mcgarrahjohn/quill"

  depends_on macos: ">= :catalina"

  app "Quill.app"

  zap trash: [
    "~/Library/Application Support/com.quill.desktop",
    "~/Library/Caches/com.quill.desktop",
    "~/Library/Logs/com.quill.desktop",
    "~/Library/Preferences/com.quill.desktop.plist",
    "~/Library/Saved Application State/com.quill.desktop.savedState",
  ]
end

class Wow < Formula
  desc "Roman Urdu programming language compiler"
  homepage "https://github.com/wow-language/wow"
  license "MIT"
  version "0.1.0"

  on_macos do
    on_arm do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-aarch64-macos.tar.gz"
      sha256 "HOMEBREW_SHA256_AARCH64_MACOS"
    end
    on_intel do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-x86_64-macos.tar.gz"
      sha256 "HOMEBREW_SHA256_X86_64_MACOS"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-aarch64-linux.tar.gz"
      sha256 "HOMEBREW_SHA256_AARCH64_LINUX"
    end
    on_intel do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-x86_64-linux.tar.gz"
      sha256 "HOMEBREW_SHA256_X86_64_LINUX"
    end
  end

  def install
    bin.install "wow"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/wow --version")
  end
end

class Wow < Formula
  desc "Roman Urdu programming language compiler"
  homepage "https://github.com/wow-language/wow"
  license "MIT"
  version "0.1.5"

  on_macos do
    on_arm do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-aarch64-macos.tar.gz"
      sha256 "ffff6a1c32546ea50b5025b70b32c7db3ac23a21fc29665f5eddbb0617c913f5"
    end
    on_intel do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-x86_64-macos.tar.gz"
      sha256 ""
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-aarch64-linux.tar.gz"
      sha256 "a58df73495c97a9d6e9dc69654395f84c830b3f946c7c89b3ddb2303d09b89fd"
    end
    on_intel do
      url "https://github.com/wow-language/wow/releases/download/v#{version}/wow-v#{version}-x86_64-linux.tar.gz"
      sha256 "bad0d764198d68590d9fcbc3de9987c7074f7cee2a3f94ac2a097053c9dc414c"
    end
  end

  def install
    bin.install "wow"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/wow --version")
  end
end

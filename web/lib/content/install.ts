/**
 * The two one-line installs, shared by the install page and the homepage
 * hero. Code-owned shell, never translated.
 */
export const INSTALL_COMMANDS = {
  shell: "curl -fsSL https://codewhale.net/install.sh | sh",
  npm: "npm install -g codewhale",
  // winget-pkgs publishes `HunterBown.CodeWhale` (portable x64, `codewhale`
  // command; checked 2026-10-04, latest 0.10.0). Scoop and the GitHub Release
  // installer are the alternatives in the install guide.
  windows: "winget install HunterBown.CodeWhale",
} as const;

export const INSTALL_COPY = {
  metaTitle: { en: "Install · Codewhale", zh: "安装 · Codewhale" },
  metaDescription: { en: "Install Codewhale on macOS, Linux, or Windows, connect a model, and run your first task. Covers package managers and source builds.", zh: "在 macOS、Linux 或 Windows 上安装 Codewhale，连接模型并运行第一项任务。也介绍包管理器和源码构建。" },
  source: { en: "Read the verified guide on GitHub", zh: "查看 GitHub 上已验证的指南" },
  translationNotice: { en: "This verified guide is in English. The Chinese translation has not yet been updated for this revision.", zh: "本页显示已验证的英文指南。中文译文尚未更新至此修订版。" },
  configuration: { en: "Settings and configuration", zh: "设置与配置" },
} as const;

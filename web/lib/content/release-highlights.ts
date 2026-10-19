export const RELEASE_HIGHLIGHTS = {
  version: "0.10.2",
  comingHeading: { en: "Coming in v{version}", zh: "v{version} 即将推出" },
  newHeading: { en: "New in v{version}", zh: "v{version} 新功能" },
  comingLede: { en: "Merged on main and not in a published release yet. Build main to try it now.", zh: "已合并到 main，尚未包含在已发布版本中。现在可从 main 构建试用。" },
  changelogLink: { en: "Read the changelog", zh: "查看更新日志" },
  items: [
    { title: "/pet on", body: { en: "Turns your terminal into an animated whale that shows what the agent is doing. The message box stays live.", zh: "把终端变成一头动态鲸鱼，显示智能体正在做什么。消息框始终可用。" } },
    { title: "/workbar terminal", body: { en: "Shows the shells the agent is running as they run. Type into one, or press Ctrl+N for your own. macOS and Linux.", zh: "实时显示智能体正在运行的 shell。可以直接在其中输入，或按 Ctrl+N 打开你自己的 shell。支持 macOS 和 Linux。" } },
    { title: "/undo", body: { en: "Takes back your whole last request in one step, including every file it changed.", zh: "一步撤回你上一次的完整请求，包括它改动的每个文件。" } },
    { title: "Plan → Work", body: { en: "When a plan ends with open steps, Codewhale asks whether to start Work with Ask or Auto-Review, or keep planning.", zh: "计划结束时如果还有未完成的步骤，Codewhale 会询问是用 Ask 或 Auto-Review 开始 Work，还是继续规划。" } },
    { title: "curl / wget", body: { en: "Outside Full Access, a curl or wget that would upload a credential never runs without your approval.", zh: "在 Full Access 之外，任何会上传凭据的 curl 或 wget 都必须经你批准才会运行。" } },
    { title: "DeepSeek Search", body: { en: "web_search can use DeepSeek's new Search API: set [search] provider = \"deepseek\" with a DeepSeek key.", zh: "web_search 可以使用 DeepSeek 新的搜索 API：在 [search] 中设置 provider = \"deepseek\" 并提供 DeepSeek 密钥。" } },
  ],
};

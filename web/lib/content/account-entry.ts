/**
 * account-entry.ts — copy for the public sign-in page.
 *
 * codewhale.net is not the signed-in app; this page sends existing users to
 * app.codewhale.net while saying plainly that new accounts are not open
 * and that the terminal needs no account.
 */

import type { LocalizedText } from "./vocabulary";

export const ACCOUNT_ENTRY_COPY = {
  signIn: {
    metaTitle: { en: "Sign in · Codewhale", zh: "登录 · Codewhale" },
    kicker: { en: "Sign in", zh: "登录" },
    title: { en: "Sign in to Codewhale.", zh: "登录 Codewhale 账户。" },
    action: { en: "Sign in", zh: "登录" },
  },
  lede: {
    en: "New Codewhale accounts are not open. If you already signed in with codewhale login, you can sign in here. The terminal and codewhale web need no account.",
    zh: "新的 Codewhale 账户暂未开放。如果你已用 codewhale login 登录过，可以在这里登录。终端和 codewhale web 无需账户。",
  },
  metaDescription: {
    en: "Sign in to an existing Codewhale account. New accounts are not open; the terminal needs no account.",
    zh: "登录已有的 Codewhale 账户。新账户暂未开放；使用终端无需账户。",
  },
  installLocally: { en: "Install locally", zh: "本机安装" },
} as const satisfies Record<string, unknown> & { lede: LocalizedText; installLocally: LocalizedText };

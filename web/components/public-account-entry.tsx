import Link from "next/link";
import { ACCOUNT_ENTRY_COPY } from "@/lib/content/account-entry";
import { pickText } from "@/lib/i18n/dictionaries";
import { publicAuthAppDestination } from "@/lib/public-auth-routes";
import { WhalePose } from "./whale-pose";

/**
 * The public sign-in door. codewhale.net is not the signed-in app: one
 * primary action sends the reader to it and one secondary action installs
 * locally (no account needed). The whale listens.
 */
export function PublicAccountEntry({
  locale,
}: {
  locale: string;
}) {
  const copy = ACCOUNT_ENTRY_COPY.signIn;
  const appHref = publicAuthAppDestination(locale);

  return (
    <div className="account-entry">
      <WhalePose pose="listen" className="account-entry-pose" priority />
      <h1 className="page-title">{pickText(copy.title, locale)}</h1>
      <p className="page-lede">{pickText(ACCOUNT_ENTRY_COPY.lede, locale)}</p>
      <div className="actions">
        <a className="btn btn-primary btn-lg" href={appHref} data-usage="login">
          {pickText(copy.action, locale)}
        </a>
        <Link className="btn btn-secondary btn-lg" href={`/${locale}/install`}>
          {pickText(ACCOUNT_ENTRY_COPY.installLocally, locale)}
        </Link>
      </div>
    </div>
  );
}

import { permanentRedirect } from "next/navigation";

export default async function ComputersDocsRedirect({
  params,
}: {
  params: Promise<{ locale: string }>;
}) {
  const { locale } = await params;
  permanentRedirect(`/${locale}/docs/fleet`);
}

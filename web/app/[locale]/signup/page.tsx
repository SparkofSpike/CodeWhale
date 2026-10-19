import { permanentRedirect } from "next/navigation";

export default async function SignUpRedirect({
  params,
}: {
  params: Promise<{ locale: string }>;
}) {
  const { locale } = await params;
  permanentRedirect(`/${locale}/signin`);
}

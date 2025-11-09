import Image from "next/image";

type SearchParams = { [key: string]: string | string[] | undefined };

export default async function Home({
  searchParams,
}: {
  searchParams: SearchParams;
}) {
  // If this request contains an OAuth `code`, exchange it server-side and show the result
  const code = Array.isArray(searchParams.code)
    ? searchParams.code[0]
    : searchParams.code;
  const state = Array.isArray(searchParams.state)
    ? searchParams.state[0]
    : searchParams.state;

  if (code) {
    const client_id = process.env.MAIL_OAUTH_CLIENT_ID;
    const client_secret = process.env.MAIL_OAUTH_CLIENT_SECRET;
    const redirect =
      process.env.MAIL_OAUTH_REDIRECT ||
      `${process.env.NEXT_PUBLIC_SITE_URL || ""}/callback`;

    if (!client_id || !client_secret) {
      return (
        <div className="p-8">
          <h1>Server misconfigured</h1>
          <p>Missing MAIL_OAUTH_CLIENT_ID or MAIL_OAUTH_CLIENT_SECRET</p>
        </div>
      );
    }

    const tokenUrl = "https://oauth2.googleapis.com/token";
    const body = new URLSearchParams({
      code,
      client_id,
      client_secret,
      redirect_uri: redirect,
      grant_type: "authorization_code",
    });

    const tokenRes = await fetch(tokenUrl, {
      method: "POST",
      headers: { "Content-Type": "application/x-www-form-urlencoded" },
      body: body.toString(),
    });

    const tokenJson = await tokenRes.json();
    const ok = tokenRes.ok;

    const safeInfo = {
      access_token_len: tokenJson.access_token
        ? String(tokenJson.access_token).length
        : 0,
      has_refresh_token: !!tokenJson.refresh_token,
      scope: tokenJson.scope || null,
      error: tokenJson.error || null,
      state: state || null,
    };

    return (
      <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
        <main className="flex min-h-screen w-full max-w-3xl flex-col items-center justify-center py-32 px-16 bg-white dark:bg-black sm:items-start">
          <Image
            className="dark:invert"
            src="/next.svg"
            alt="Next.js logo"
            width={100}
            height={20}
            priority
          />
          <div className="w-full max-w-2xl">
            <h1 className="text-2xl font-semibold">OAuth callback received</h1>
            <p className="mt-2">Status: {ok ? "OK" : "ERROR"}</p>
            <pre className="mt-4 bg-gray-100 p-4 rounded">{JSON.stringify(safeInfo, null, 2)}</pre>
            <p className="mt-4">You can close this window.</p>
            <button
              id="copy"
              className="mt-4 inline-flex items-center gap-2 rounded bg-black text-white px-3 py-2"
              onClick={undefined}
            >
              Copy full response to clipboard
            </button>
            <pre id="full" style={{ display: "none" }}>
              {JSON.stringify(tokenJson, null, 2)}
            </pre>
            <script
              dangerouslySetInnerHTML={{
                __html: `
                  document.addEventListener('DOMContentLoaded', function(){
                    const btn = document.getElementById('copy');
                    if(!btn) return;
                    btn.addEventListener('click', async () => {
                      try {
                        const txt = document.getElementById('full').textContent;
                        await navigator.clipboard.writeText(txt);
                        alert('Token JSON copied to clipboard');
                      } catch (e) {
                        alert('Copy failed');
                      }
                    });
                  });
                `,
              }}
            />
          </div>
        </main>
      </div>
    );
  }

  // default page when not handling callback
  return (
    <div className="flex min-h-screen items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex min-h-screen w-full max-w-3xl flex-col items-center justify-between py-32 px-16 bg-white dark:bg-black sm:items-start">
        <Image
          className="dark:invert"
          src="/next.svg"
          alt="Next.js logo"
          width={100}
          height={20}
          priority
        />
        <div className="flex flex-col items-center gap-6 text-center sm:items-start sm:text-left">
          <h1 className="max-w-xs text-3xl font-semibold leading-10 tracking-tight text-black dark:text-zinc-50">
            To get started, edit the page.tsx file.
          </h1>
          <p className="max-w-md text-lg leading-8 text-zinc-600 dark:text-zinc-400">
            Looking for a starting point or more instructions? Head over to{" "}
            <a
              href="https://vercel.com/templates?framework=next.js&utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
              className="font-medium text-zinc-950 dark:text-zinc-50"
            >
              Templates
            </a>{" "}
            or the{" "}
            <a
              href="https://nextjs.org/learn?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
              className="font-medium text-zinc-950 dark:text-zinc-50"
            >
              Learning
            </a>{" "}
            center.
          </p>
        </div>
        <div className="flex flex-col gap-4 text-base font-medium sm:flex-row">
          <a
            className="flex h-12 w-full items-center justify-center gap-2 rounded-full bg-foreground px-5 text-background transition-colors hover:bg-[#383838] dark:hover:bg-[#ccc] md:w-[158px]"
            href="https://vercel.com/new?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            <Image
              className="dark:invert"
              src="/vercel.svg"
              alt="Vercel logomark"
              width={16}
              height={16}
            />
            Deploy Now
          </a>
          <a
            className="flex h-12 w-full items-center justify-center rounded-full border border-solid border-black/[.08] px-5 transition-colors hover:border-transparent hover:bg-black/[.04] dark:border-white/[.145] dark:hover:bg-[#1a1a1a] md:w-[158px]"
            href="https://nextjs.org/docs?utm_source=create-next-app&utm_medium=appdir-template-tw&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            Documentation
          </a>
        </div>
      </main>
    </div>
  );
}

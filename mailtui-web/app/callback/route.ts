import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const url = new URL(req.url);
  const code = url.searchParams.get("code");
  const state = url.searchParams.get("state");

  if (!code) {
    return new NextResponse("Missing code", { status: 400 });
  }

  const client_id = process.env.MAIL_OAUTH_CLIENT_ID;
  const client_secret = process.env.MAIL_OAUTH_CLIENT_SECRET;
  const redirect = process.env.MAIL_OAUTH_REDIRECT || `${process.env.NEXT_PUBLIC_SITE_URL || ""}/callback`;

  if (!client_id || !client_secret) {
    return new NextResponse("Server misconfigured: missing client id/secret", { status: 500 });
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

  // render a minimal HTML page that shows status and token info
  const ok = tokenRes.ok;
  const safeInfo = {
    access_token_len: tokenJson.access_token ? String(tokenJson.access_token).length : 0,
    has_refresh_token: !!tokenJson.refresh_token,
    scope: tokenJson.scope || null,
    error: tokenJson.error || null,
    state: state || null,
  };

  const html = `<!doctype html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>MailTUI OAuth callback</title>
    <meta name="viewport" content="width=device-width,initial-scale=1" />
    <style>
      body{font-family:system-ui,Segoe UI,Roboto,-apple-system,Arial;margin:40px;color:#111}
      .card{max-width:720px;padding:24px;border:1px solid #e5e7eb;border-radius:8px}
      pre{background:#f3f4f6;padding:12px;border-radius:6px;overflow:auto}
      button{margin-top:12px;padding:8px 12px;border-radius:6px;border:none;background:#111;color:#fff;cursor:pointer}
    </style>
  </head>
  <body>
    <div class="card">
      <h1>OAuth callback received</h1>
      <p>Status: ${ok ? "OK" : "ERROR"}</p>
      <pre>${JSON.stringify(safeInfo, null, 2)}</pre>
      <p>You can close this window.</p>
      <button id="copy">Copy full response to clipboard</button>
      <pre id="full" style="display:none">${JSON.stringify(tokenJson, null, 2)}</pre>
      <script>
        document.getElementById('copy').addEventListener('click', async () => {
          try {
            const txt = document.getElementById('full').textContent;
            await navigator.clipboard.writeText(txt);
            alert('Token JSON copied to clipboard');
          } catch (e) {
            alert('Copy failed');
          }
        });
      </script>
    </div>
  </body>
</html>`;

  return new NextResponse(html, {
    status: ok ? 200 : 500,
    headers: { "content-type": "text/html; charset=utf-8" },
  });
}
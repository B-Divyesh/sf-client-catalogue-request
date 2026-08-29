export const ENTRA_CLIENT_ID = '25c704f4-465a-47af-80ab-2c489466b697';
export const ENTRA_TENANT_ID = '35c6fe40-0ec0-46b6-98c6-213ad4de6650';
export const ENTRA_AUTHORITY = `https://sociobotcustomers.ciamlogin.com/${ENTRA_TENANT_ID}/`;
export const ENTRA_API_SCOPE = `api://${ENTRA_CLIENT_ID}/access_as_user`;

type Msal = import('@azure/msal-browser').PublicClientApplication;

let clientPromise: Promise<Msal> | undefined;
let redirectPromise: ReturnType<Msal['handleRedirectPromise']> | undefined;

async function client(): Promise<Msal> {
  if (!clientPromise) {
    clientPromise = import('@azure/msal-browser').then(async ({ PublicClientApplication }) => {
      const instance = new PublicClientApplication({
        auth: {
          clientId: ENTRA_CLIENT_ID,
          authority: ENTRA_AUTHORITY,
          redirectUri: `${location.origin}/auth/callback`,
        },
        cache: { cacheLocation: 'sessionStorage' },
      });
      await instance.initialize();
      return instance;
    });
  }
  return clientPromise;
}

async function redirectResult() {
  const instance = await client();
  redirectPromise ??= instance.handleRedirectPromise();
  return redirectPromise;
}

export async function getSellerAccessToken(): Promise<string | null> {
  const instance = await client();
  const returned = await redirectResult();
  if (returned?.accessToken) return returned.accessToken;
  const account = returned?.account ?? instance.getAllAccounts()[0];
  if (!account) return null;
  try {
    return (await instance.acquireTokenSilent({ account, scopes: [ENTRA_API_SCOPE] })).accessToken || null;
  } catch {
    return null;
  }
}

export async function beginSellerSignIn(): Promise<void> {
  const instance = await client();
  await instance.loginRedirect({ scopes: [ENTRA_API_SCOPE] });
}

export async function endSellerSession(): Promise<void> {
  const instance = await client();
  const account = instance.getAllAccounts()[0];
  if (account) {
    await instance.logoutRedirect({ account, postLogoutRedirectUri: `${location.origin}/manage` });
  }
}

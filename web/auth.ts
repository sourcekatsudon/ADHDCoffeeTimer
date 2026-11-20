import NextAuth from 'next-auth';
import Google from 'next-auth/providers/google';

export const { handlers, auth, signIn, signOut } = NextAuth({
  trustHost: true,
  session: { strategy: 'jwt' },
  providers: [Google({
    authorization: { params: { scope: 'openid email profile' } },
  })],
  callbacks: {
    async signIn({ account, profile }) {
      return account?.provider === 'google'
        && (profile as { email_verified?: boolean } | undefined)?.email_verified === true;
    },
    async jwt({ token, account }) {
      if (account?.provider === 'google') {
        (token as typeof token & { googleSub?: string }).googleSub = account.providerAccountId;
      }
      return token;
    },
    async session({ session, token }) {
      (session.user as typeof session.user & { id?: string }).id =
        (token as typeof token & { googleSub?: string }).googleSub ?? token.sub ?? '';
      return session;
    },
  },
});

export async function currentAccount(): Promise<{ id: string; name: string; email: string } | null> {
  const session = await auth();
  const user = session?.user as (typeof session.user & { id?: string }) | undefined;
  if (!user?.id) return null;
  return { id: user.id, name: user.name ?? '', email: user.email ?? '' };
}

import type { Metadata } from 'next';
import './globals.css';
export const metadata: Metadata = {
  title: 'ADHD Coffee Time | Focus History',
  description: 'A private seven-day focus activity dashboard that merges your connected PCs.',
};
export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="ja"><body>{children}</body></html>;
}

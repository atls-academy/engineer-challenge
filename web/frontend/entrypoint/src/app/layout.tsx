import type { Metadata } from 'next'
import { StoreProvider } from '@store/root'

export const metadata: Metadata = {
  title: 'App',
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="ru">
      <body>
        <StoreProvider>{children}</StoreProvider>
      </body>
    </html>
  )
}

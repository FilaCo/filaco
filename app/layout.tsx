import { ReactNode } from 'react'

const RootLayout = ({ children }: Readonly<{ children: ReactNode }>) => (
  <html lang="en">
    <body>{children}</body>
  </html>
)

export default RootLayout

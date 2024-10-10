'use client'

import { ReactNode } from 'react'
import { AntdRegistry } from '@ant-design/nextjs-registry'

import '@/app/globals.css'
import { inter } from './lib/fonts'
import { ConfigProvider, Layout } from 'antd'
import { theme } from './lib/theme'

const { Content, Footer, Header } = Layout

const RootLayout = ({ children }: Readonly<{ children: ReactNode }>) => (
  <html lang='en' className={inter.className}>
    <body>
      <AntdRegistry>
        <ConfigProvider theme={theme}>
          <Layout>
            <Header>Header</Header>
            <Content>{children}</Content>
            <Footer>Footer</Footer>
          </Layout>
        </ConfigProvider>
      </AntdRegistry>
    </body>
  </html>
)

export default RootLayout

import { Inter } from 'next/font/google'
import localFont from 'next/font/local'

export const inter = Inter({
  subsets: ['latin', 'cyrillic'],
  variable: '--font-inter',
  display: 'swap',
})

export const jetbrains_mono = localFont({
  src: './fonts/JetBrainsMono[wght].ttf',
  variable: '--font-jetbrains-mono',
  display: 'swap',
})

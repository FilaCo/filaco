import { inter, jetbrains_mono } from "@/app/lib/fonts"
import classNames from "classnames"
import { ReactNode } from "react"

export const typographyVariants = ['h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'p', 'code', 'span'] as const
export type TypographyVariant = typeof typographyVariants[number]

export interface TypographyProps {
  children: ReactNode
  variant?: TypographyVariant
  className?: string
}

export const Typography = ({ children, variant, className, ...other }: TypographyProps) => {
  const Component = variant || 'span'
  const typographyClass = classNames(
    className,
    {
      [inter.className]: variant !== 'code',
      [jetbrains_mono.className]: variant === 'code'
    }
  )

  return <Component className={typographyClass} {...other}>{children}</Component>
}

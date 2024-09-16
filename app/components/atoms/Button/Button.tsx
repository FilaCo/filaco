export interface ButtonProps {
  children: React.ReactNode
}

export const Button = ({ children, ...other }: ButtonProps) => {
  return <button {...other}>{children}</button>
}

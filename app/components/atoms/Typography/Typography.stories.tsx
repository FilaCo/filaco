import type { Meta, StoryObj } from '@storybook/react';

import { Typography, typographyVariants } from './Typography';

const meta: Meta<typeof Typography> = {
  component: Typography,
};

export default meta;

type Story = StoryObj<typeof Typography>;

export const Basic: Story = {
  args: {
    variant: 'p',
    children: 'Hello, world!'
  },
};

export const Examples: Story = {
  render: (args) => {
    const elems = typographyVariants.map((variant) => {
      const annotation = `<Typography variant='${variant}'>Hello, ${variant}!</Typography>`

      return <li>
        <Typography variant='code'>{annotation}</Typography>
        <br />
        <Typography variant={variant}>Hello, {variant}!</Typography>
      </li>
    })

    return <ul>{elems}</ul>
  }
};
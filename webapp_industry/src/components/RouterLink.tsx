import * as React from 'react'
import { createLink, type LinkComponent } from '@tanstack/react-router'
import { NavLink, type NavLinkProps } from '@mantine/core'

interface RouterLinkProps extends Omit<NavLinkProps, 'href'> {
  // Add any additional props you want to pass to the anchor
}

const MantineLinkComponent = React.forwardRef<
  HTMLAnchorElement,
  RouterLinkProps
>((props, ref) => {
    return <NavLink
        ref={ref}
        {...props}
    />
})

const CreatedLinkComponent = createLink(MantineLinkComponent)

export const CustomLink: LinkComponent<typeof MantineLinkComponent> = (
  props,
) => {
    return <CreatedLinkComponent
        preload="intent"
        {...props}
    />
}

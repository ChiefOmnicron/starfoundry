

import type { ReactElement } from "react";

export function LoginButton({
    height = '30px',

    onClick,
}: LoginButtonProp): ReactElement {
    return <img
        onClick={onClick}
        src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
        style={{
            cursor: 'pointer',
            height: height,
        }}
    />
}

export type LoginButtonProp = {
    height?: string;

    onClick: () => void;
}

import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/login')({
    component: Login,
})

export function Login() {
    const redirect = `${window.location.origin}/project-groups/`

    return <img
        onClick={() => { window.location.href = `/api/auth/login?redirect=${redirect}`}}
        src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
        style={{
            cursor: 'pointer'
        }}
    />
}

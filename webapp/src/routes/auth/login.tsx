import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/auth/login')({
    component: Login,
})

export function Login() {
    return <img
        onClick={() => { window.location.href = "/api/auth/login"}}
        src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
        style={{
            cursor: 'pointer'
        }}
    />
}

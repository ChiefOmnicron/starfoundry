import { axiosClient } from '@starfoundry/components/services/client'
import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/characters/')({
    component: RouteComponent,
})

function RouteComponent() {
    const loginCorporation = async () => {
        (await axiosClient())
            .get('/api/auth/login/corporation')
            .then((x: any) => {
                console.log(x)
                window.location.href = x.data.url;
            })
            .catch(e => {
                console.error(e)
            })
    }

    return <img
        //onClick={() => { window.location.href = "/api/auth/login/corporation"}}
        onClick={() => loginCorporation()}
        src="https://web.ccpgamescdn.com/eveonlineassets/developers/eve-sso-login-black-large.png"
        style={{
            cursor: 'pointer'
        }}
    />
}

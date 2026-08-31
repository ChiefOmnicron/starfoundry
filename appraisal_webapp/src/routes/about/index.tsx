import { Text } from '@mantine/core';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/about/')({
    component: RouteComponent,
});

function RouteComponent() {
    return <div>
        <h1>About</h1>

        <h2>Is there an API</h2>
        <span>
            Yes you can find the documentation under "". Please use it sensibly, I have no interest in limiting it, but I will if necessary.
        </span>

        <h2>Contact information</h2>
        <span>
            If you have problems, ideas, or want to get in contact, feel free to checkout the following links
        </span>
        <ul>
            <li>
                Discord
                <a href="https://discord.gg/qShbyn4r9N">https://discord.gg/qShbyn4r9N</a>
            </li>
            <li>
                GitHub
                <a href="https://github.com/ChiefOmnicron/starfoundry">https://github.com/ChiefOmnicron/starfoundry</a>
            </li>
            <li>
                In-Game mail to 'Eistonen Kodan Sasen'
            </li>
        </ul>

        <h2>Donations</h2>
        <span>
            <Text>Feel free to donate to the in-game corporation: "StarFoundry", ticker: "[-S.F.]"</Text>
        </span>
    </div>
}

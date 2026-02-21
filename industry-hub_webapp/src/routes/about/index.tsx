import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/about/')({
    component: RouteComponent,
});

function RouteComponent() {
    return <div>
        <h1>About</h1>

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
        </ul>
    </div>
}

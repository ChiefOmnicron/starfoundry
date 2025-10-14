import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/contact/')({
    component: About,
});

function About() {
    return <div>
        <h1>Contact</h1>

        <span>
            You can choose between several communication channels:
        </span>

        <ol>
            <ul>- Join RCI's <a href='https://discord.gg/yFxsjw9' target='_blank'>Discord</a> and send you question in the general channel</ul>
            <ul>- Contact 'chiefomnicron' on Discord</ul>
            <ul>- Send 'Eistonen Kodan Sasen' a Slack message (for BRAVE)</ul>
            <ul>- Send 'Eistonen Kodan Sasen' an GSF forum message</ul>
        </ol>
    </div>
}

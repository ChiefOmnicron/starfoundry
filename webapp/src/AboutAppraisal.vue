<template>
    <div>
        <h1>What is this</h1>
        <span>It's an Eve Online tool, that calculates how much your items are worth, based on current market data.</span>

        <h1>How are the prices calculated</h1>
        <span>Every ~5 minutes, the market data for all markets are fetched and stored in a database. When you create an appraisal it will take the latest data that is stored. For the buy value, the entry with the highest buy is selected, and then multiplied with the amount of items you have. For sell the lowest value is selected, and then multiplied with your number of items.</span>

        <h1>What is the compression tab</h1>
        <span>Besides calculating the worth of your materials, the tool can also calculate the amount of compressed ore you need to fulfill a given amount of minerals. The same works also for gas.</span>

        <h1>What is the reprocessing tab</h1>
        <span>The inverse of compression, calculate how much the ores, or items are worth reprocessed.</span>

        <h1>Is there an API</h1>
        <span>Yes, you can find the documentation under <a href="https://industry.starfoundry.space/api/v1">https://industry.starfoundry.space/api/v1</a>. Please use it sensibly, I have no interest in limiting it, but I will if necessary.</span>

        <h1>Can I host my own version of it</h1>
        <span>Yes, but that part is still in development, it will be available soon tm.</span>

        <h1>Contact information</h1>
        <span>If you have problems, or ideas, feel free to checkout the foillowing links</span>
        <ul>
            <li>
                Discord <a href="https://discord.gg/qShbyn4r9N">https://discord.gg/qShbyn4r9N</a>
            </li>
            <li>
                GitHub <a href="https://github.com/ChiefOmnicron/starfoundry">https://github.com/ChiefOmnicron/starfoundry</a>
            </li>
            <li>
                Ingame mail to 'Eistonen Kodan Sasen'
            </li>
        </ul>
    </div>
</template>

